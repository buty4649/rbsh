use super::{
    context::Context,
    exec::{
        syscall::{SysCallResult, SysCallWrapper, Wrapper},
        SHELL_FDBASE,
    },
    status::ExitStatus,
};

use nix::{
    sys::{
        signal::{killpg, SaFlags, SigAction, SigHandler, SigSet, Signal},
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};
use signal_hook::{
    consts::signal::*, iterator::backend::PollResult, iterator::backend::RefSignalIterator,
    iterator::backend::SignalDelivery, iterator::exfiltrator::SignalOnly,
};
use std::io::Error as IoError;
use std::io::Read;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc, Condvar, Mutex,
};
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

struct Handle {
    sf_handle: signal_hook::iterator::Handle,
    read: RawFd,
    write: RawFd,
}

impl Handle {
    fn close(&self) {
        self.sf_handle.close();
        nix::unistd::close(self.read).unwrap();
        nix::unistd::close(self.write).unwrap();
    }
}

struct SignalHandler {
    delivery: SignalDelivery<UnixStream, SignalOnly>,
    read: RawFd,
    write: RawFd,
}

impl SignalHandler {
    fn new(signals: Vec<nix::libc::c_int>) -> Result<Self, IoError> {
        Self::new_at(Wrapper::new(), signals)
    }

    fn new_at(wrapper: Wrapper, signals: Vec<nix::libc::c_int>) -> Result<Self, IoError> {
        let (stream_read, stream_write) = UnixStream::pair()?;
        let read = wrapper
            .dup_fd(stream_read.as_raw_fd(), SHELL_FDBASE)
            .unwrap();
        let write = wrapper
            .dup_fd(stream_write.as_raw_fd(), SHELL_FDBASE)
            .unwrap();
        let stream_read = unsafe { UnixStream::from_raw_fd(read) };
        let stream_write = unsafe { UnixStream::from_raw_fd(write) };
        let delivery =
            SignalDelivery::with_pipe(stream_read, stream_write, SignalOnly::default(), signals)?;

        Ok(Self {
            delivery,
            read,
            write,
        })
    }

    fn handle(&self) -> Handle {
        Handle {
            sf_handle: self.delivery.handle(),
            read: self.read,
            write: self.write,
        }
    }

    fn has_signals(read: &mut UnixStream) -> Result<bool, IoError> {
        loop {
            match read.read(&mut [0u8]) {
                Ok(num_read) => break Ok(num_read > 0),
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::Interrupted {
                        break Err(e);
                    }
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a mut SignalHandler {
    type Item = nix::libc::c_int;
    type IntoIter = Forever<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Forever(RefSignalIterator::new(&mut self.delivery))
    }
}

struct Forever<'a>(RefSignalIterator<'a, UnixStream, SignalOnly>);
impl<'a> Iterator for Forever<'a> {
    type Item = nix::libc::c_int;

    fn next(&mut self) -> Option<nix::libc::c_int> {
        match self.0.poll_signal(&mut SignalHandler::has_signals) {
            PollResult::Signal(result) => Some(result),
            PollResult::Closed => None,
            PollResult::Pending => unreachable![],
            PollResult::Err(e) => panic!("Unexpected error: {}", e),
        }
    }
}

pub struct JobSignalHandler {
    inner: Arc<(Mutex<JobSignalHandlerInner>, Condvar)>,
    forground: Arc<AtomicI32>,
    handle: Handle,
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
        let forground = Arc::new(AtomicI32::new(0));

        let mut signals = SignalHandler::new(vec![SIGINT, SIGCHLD])?;
        let handle = signals.handle();

        let pair = inner.clone();
        let fg = forground.clone();
        thread::spawn(move || {
            for sig in &mut signals {
                match sig {
                    SIGINT => {
                        let (inner, cvar) = &*pair;
                        inner.lock().unwrap().set_interrupt_flag();
                        cvar.notify_one();

                        let f = fg.load(Ordering::Relaxed);
                        if f > 0 {
                            let pgid = Pid::from_raw(f);
                            killpg(pgid, Signal::SIGINT).ok();
                            fg.store(0, Ordering::Relaxed);
                        }
                    }
                    SIGCHLD => {
                        let any_child = Pid::from_raw(-1);
                        loop {
                            match waitpid(any_child, Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
                                Ok(s) if s == WaitStatus::StillAlive => break,
                                Err(_) => break,
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
                            }
                        }
                    }
                    _ => unreachable![],
                }
            }
        });

        Ok(Self {
            inner,
            forground,
            handle,
        })
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

    pub fn set_forground(&mut self, pid: Pid) {
        self.forground.store(pid.as_raw(), Ordering::Relaxed);
    }

    pub fn reset_forground(&mut self) {
        self.forground.store(0, Ordering::Relaxed);
    }

    pub fn close(&mut self) {
        self.handle.close();
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
