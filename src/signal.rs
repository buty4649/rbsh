use super::{
    context::Context,
    exec::syscall::{SysCallResult, SysCallWrapper, Wrapper},
    status::ExitStatus,
};

use nix::{
    sys::{
        signal::{SaFlags, SigAction, SigHandler, SigSet, Signal},
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};
use signal_hook::{consts::signal::*, iterator::Signals};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

static TTYSIGNALS: [i32; 5] = [SIGQUIT, SIGTERM, SIGTSTP, SIGTTIN, SIGTTOU];

pub fn recognize_sigpipe(ctx: &Context) -> SysCallResult<()> {
    // Ignore SIGPIPE by default
    // https://github.com/rust-lang/rust/pull/13158
    let sa = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
    ctx.wrapper().sigaction(Signal::SIGPIPE, &sa)?;
    Ok(())
}

pub fn ignore_tty_signals(ctx: &Context) -> SysCallResult<()> {
    let sa = SigAction::new(SigHandler::SigIgn, SaFlags::empty(), SigSet::empty());
    for sig in TTYSIGNALS {
        let sig = Signal::try_from(sig).unwrap();
        ctx.wrapper().sigaction(sig, &sa)?;
    }
    Ok(())
}

pub fn restore_tty_signals(wrapper: &Wrapper) -> SysCallResult<()> {
    let sa = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
    for sig in TTYSIGNALS {
        let sig = Signal::try_from(sig).unwrap();
        wrapper.sigaction(sig, &sa)?;
    }
    Ok(())
}

pub struct JobSignalHandler {
    inner: Arc<(Mutex<JobSignalHandlerInner>, Condvar)>,
}

pub struct JobSignalHandlerInner {
    interrupt: bool,
    status: Vec<WaitStatus>,
}

impl JobSignalHandlerInner {
    pub fn new() -> Self {
        Self {
            interrupt: false,
            status: vec![],
        }
    }

    pub fn set_interrupt_flag(&mut self) {
        self.interrupt = true
    }

    pub fn reset_interrupt_flag(&mut self) -> bool {
        let ret = self.interrupt;
        self.interrupt = false;
        ret
    }

    pub fn push_status(&mut self, s: WaitStatus) {
        self.status.push(s)
    }

    pub fn has_pid(&self, pid: Pid) -> bool {
        self.find_pid(pid).is_some()
    }

    pub fn find_pid(&self, pid: Pid) -> Option<usize> {
        self.status
            .iter()
            .enumerate()
            .find(|(_, status)| match *status {
                WaitStatus::Exited(p, _) if *p == pid => true,
                WaitStatus::Signaled(p, _, _) if *p == pid => true,
                _ => false,
            })
            .map_or(None, |(index, _)| Some(index))
    }

    pub fn remove_status(&mut self, pid: Pid) -> Option<WaitStatus> {
        self.find_pid(pid).map_or(None, |index| {
            let ret = self.status[index];
            self.status.remove(index);
            Some(ret)
        })
    }
}

impl JobSignalHandler {
    pub fn start() -> Result<Self, std::io::Error> {
        let inner = Arc::new((Mutex::new(JobSignalHandlerInner::new()), Condvar::new()));

        let mut signals = Signals::new(vec![SIGINT, SIGCHLD])?;
        signals.handle();

        let pair = inner.clone();
        thread::spawn(move || {
            for sig in &mut signals {
                match sig {
                    SIGINT => {
                        let (inner, cvar) = &*pair;
                        inner.lock().unwrap().set_interrupt_flag();
                        cvar.notify_one();
                    }
                    SIGCHLD => {
                        let any_child = Pid::from_raw(-1);
                        match waitpid(any_child, None) {
                            Ok(s) => {
                                let (inner, cvar) = &*pair;

                                let mut lock = inner.lock().unwrap();
                                lock.push_status(s);
                                if matches!(s, WaitStatus::Signaled(_, signal, _) if signal == Signal::SIGINT)
                                {
                                    lock.set_interrupt_flag();
                                }
                                cvar.notify_one();
                            }
                            Err(e) => {
                                eprintln!("waitpid: {}", e.desc())
                            }
                        }
                    }
                    _ => unreachable![],
                }
            }
        });

        Ok(Self { inner })
    }

    pub fn wait_for(&mut self, pid: Pid, block: bool) -> Option<ExitStatus> {
        let (mutex, cvar) = &*self.inner;
        let mut list = if block {
            cvar.wait_while(mutex.lock().unwrap(), |inner| !inner.has_pid(pid))
        } else {
            mutex.lock()
        }
        .unwrap();

        match list.remove_status(pid) {
            None => None,
            Some(status) => match status {
                WaitStatus::Exited(_, code) => Some(ExitStatus::new(code)),
                WaitStatus::Signaled(_, signal, _) => Some(ExitStatus::signaled(signal)),
                _ => unreachable![],
            },
        }
    }

    pub fn is_interrupt(&mut self) -> bool {
        let (mutex, _) = &*self.inner;
        mutex.lock().unwrap().reset_interrupt_flag()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::exec::syscall::Wrapper;
    use mockall::predicate::{eq, function};

    pub fn mock_ignore_tty_signals(mock: &mut Wrapper) {
        for sig in TTYSIGNALS {
            let sig = Signal::try_from(sig).unwrap();
            mock.expect_sigaction()
                .times(1)
                .with(
                    eq(sig),
                    function(|&sa: &SigAction| {
                        sa.handler() == SigHandler::SigIgn && sa.flags().is_empty()
                        //&& sa.mask().is_empty() I don't know how to do this.
                    }),
                )
                .return_const(Ok(SigAction::new(
                    SigHandler::SigDfl,
                    SaFlags::empty(),
                    SigSet::empty(),
                )));
        }
    }

    pub fn mock_restore_tty_signals(mock: &mut Wrapper) {
        for sig in TTYSIGNALS {
            let sig = Signal::try_from(sig).unwrap();
            mock.expect_sigaction()
                .times(1)
                .with(
                    eq(sig),
                    function(|&sa: &SigAction| {
                        sa.handler() == SigHandler::SigDfl && sa.flags().is_empty()
                        //&& sa.mask().is_empty() I don't know how to do this.
                    }),
                )
                .return_const(Ok(SigAction::new(
                    SigHandler::SigDfl,
                    SaFlags::empty(),
                    SigSet::empty(),
                )));
        }
    }

    #[test]
    fn test_ignore_tty_signals() {
        let mut mock = Wrapper::new();
        mock_ignore_tty_signals(&mut mock);
        let ctx = Context::new(mock);
        ignore_tty_signals(&ctx).unwrap();
    }

    #[test]
    fn test_reset_tty_signals() {
        let mut mock = Wrapper::new();
        mock_restore_tty_signals(&mut mock);
        restore_tty_signals(&mock).unwrap();
    }
}
