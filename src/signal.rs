use super::{
    exec::SHELL_FDBASE,
    status::ExitStatus,
    syscall::{self, SysCallResult},
};

use nix::{
    errno::errno,
    libc,
    sys::{
        signal::{killpg, SaFlags, SigAction, SigHandler, SigSet, Signal},
        wait::{waitpid, WaitStatus},
    },
    unistd::{close, Pid},
};
use once_cell::sync::OnceCell;
use signal_hook::{
    consts::signal::*,
    iterator::{
        backend::{PollResult, RefSignalIterator, SignalDelivery},
        exfiltrator::SignalOnly,
        Handle,
    },
};
use std::{
    io::{Error as IoError, Read},
    mem,
    os::unix::{
        io::{AsRawFd, FromRawFd, RawFd},
        net::UnixStream,
    },
    ptr,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
};

const TTYSIGNALS: [i32; 5] = [SIGQUIT, SIGTERM, SIGTSTP, SIGTTIN, SIGTTOU];

pub fn recognize_sigpipe() -> SysCallResult<()> {
    // Ignore SIGPIPE by default
    // https://github.com/rust-lang/rust/pull/13158
    let sa = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
    syscall::sigaction(Signal::SIGPIPE, &sa)?;
    Ok(())
}

pub fn ignore_tty_signals() -> SysCallResult<()> {
    let sa = SigAction::new(SigHandler::SigIgn, SaFlags::empty(), SigSet::empty());
    for sig in TTYSIGNALS {
        let sig = Signal::try_from(sig).unwrap();
        syscall::sigaction(sig, &sa)?;
    }
    Ok(())
}

pub fn restore_tty_signals() -> SysCallResult<()> {
    let sa = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
    for sig in TTYSIGNALS {
        let sig = Signal::try_from(sig).unwrap();
        syscall::sigaction(sig, &sa)?;
    }
    Ok(())
}

static mut SIGNAL_HANDLER_FD: OnceCell<(RawFd, RawFd)> = OnceCell::new();

struct SignalHandler {
    delivery: SignalDelivery<UnixStream, SignalOnly>,
}

impl SignalHandler {
    fn new(signals: Vec<nix::libc::c_int>) -> Result<Self, IoError> {
        Self::new_at(signals)
    }

    fn new_at(signals: Vec<nix::libc::c_int>) -> Result<Self, IoError> {
        let (stream_read, stream_write) = UnixStream::pair()?;
        let read = syscall::dup_fd(stream_read.as_raw_fd(), SHELL_FDBASE).unwrap();
        let write = syscall::dup_fd(stream_write.as_raw_fd(), SHELL_FDBASE).unwrap();
        unsafe { SIGNAL_HANDLER_FD.set((read, write)).unwrap() };
        let stream_read = unsafe { UnixStream::from_raw_fd(read) };
        let stream_write = unsafe { UnixStream::from_raw_fd(write) };
        let delivery =
            SignalDelivery::with_pipe(stream_read, stream_write, SignalOnly::default(), signals)?;

        Ok(Self { delivery })
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

    fn handle(&self) -> Handle {
        self.delivery.handle()
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
            PollResult::Closed | PollResult::Pending => None,
            PollResult::Err(e) => panic!("Unexpected error: {e}"),
        }
    }
}

pub fn close_signal_handler() {
    if let Some((read, write)) = unsafe { SIGNAL_HANDLER_FD.take() } {
        close(read).unwrap();
        close(write).unwrap();
    }
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

    pub fn get_interrupt_flag(&mut self) -> bool {
        self.interrupt
    }

    pub fn reset_interrupt_flag(&mut self) {
        self.interrupt = false
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
            .map(|(index, _)| index)
    }

    pub fn remove_status(&mut self, pid: Pid) -> Option<WaitStatus> {
        self.find_pid(pid).map(|index| {
            let ret = self.status[index];
            self.status.remove(index);
            ret
        })
    }
}

pub struct JobSignalHandler {
    inner: Arc<(Mutex<JobSignalHandlerInner>, Condvar)>,
    forground: Arc<AtomicI32>,
    signal_handler: Handle,
    //thread: std::cell::RefCell<thread::JoinHandle<()>>,
    thread: thread::JoinHandle<()>,
}

impl JobSignalHandler {
    pub fn start() -> Result<Self, std::io::Error> {
        let inner = Arc::new((Mutex::new(JobSignalHandlerInner::new()), Condvar::new()));
        let forground = Arc::new(AtomicI32::new(0));

        let mut signals = SignalHandler::new(vec![SIGINT, SIGCHLD])?;
        let signal_handler = signals.handle();
        let pair = inner.clone();
        let fg = forground.clone();

        let thread = thread::Builder::new()
        .spawn(move || {
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
        }).unwrap();

        Ok(Self {
            inner,
            forground,
            signal_handler,
            thread,
        })
    }

    pub fn wait_for(&mut self, pid: Pid, block: bool) -> Option<ExitStatus> {
        let (mutex, cvar) = &*self.inner;
        let mut list = match block {
            true => cvar.wait_while(mutex.lock().unwrap(), |inner| {
                !inner.get_interrupt_flag() && !inner.has_pid(pid)
            }),
            false => mutex.lock(),
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

    pub fn is_interrupt(&self) -> bool {
        let (mutex, _) = &*self.inner;
        mutex.lock().unwrap().get_interrupt_flag()
    }

    pub fn reset_interrupt_flag(&mut self) {
        let (mutex, _) = &*self.inner;
        mutex.lock().unwrap().reset_interrupt_flag()
    }

    pub fn set_forground(&mut self, pid: Pid) {
        self.forground.store(pid.as_raw(), Ordering::Relaxed);
    }

    pub fn reset_forground(&mut self) {
        self.forground.store(0, Ordering::Relaxed);
    }

    pub fn close(self) {
        close_signal_handler();
        self.signal_handler.close();
        self.thread.join().unwrap();
    }
}

macro_rules! sigaction {
    ($sig: expr, $new: expr, $old: expr) => {
        match libc::sigaction($sig, $new, $old) {
            -1 => Err(IoError::from_raw_os_error(errno())),
            r => Ok(r),
        }
    };
}

pub fn change_sa_restart_flag(flag: bool) -> Result<(), IoError> {
    unsafe {
        let mut sa: libc::sigaction = mem::zeroed();
        sigaction!(Signal::SIGINT as libc::c_int, ptr::null(), &mut sa)?;

        sa.sa_flags = match flag {
            true => sa.sa_flags | libc::SA_RESTART,
            false => sa.sa_flags & !libc::SA_RESTART,
        };
        sigaction!(Signal::SIGINT as libc::c_int, &sa, ptr::null_mut())?;
    };
    Ok(())
}

pub fn reset_signal_handler() -> Result<(), IoError> {
    unsafe {
        let mut sa: libc::sigaction = mem::zeroed();
        sa.sa_flags = libc::SA_RESETHAND;
        sigaction!(Signal::SIGINT as libc::c_int, &sa, ptr::null_mut())?;
        sigaction!(Signal::SIGCHLD as libc::c_int, &sa, ptr::null_mut())?;
    };

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::{ignore_tty_signals, restore_tty_signals, SaFlags, SigAction, SigHandler, SigSet};
    use crate::syscall;

    #[test]
    fn test_ignore_tty_signals() {
        let ctx = syscall::sigaction_context();
        ctx.expect().return_const(Ok(SigAction::new(
            SigHandler::SigDfl,
            SaFlags::empty(),
            SigSet::empty(),
        )));
        ignore_tty_signals().unwrap();
    }

    #[test]
    fn test_reset_tty_signals() {
        let ctx = syscall::sigaction_context();
        ctx.expect().return_const(Ok(SigAction::new(
            SigHandler::SigDfl,
            SaFlags::empty(),
            SigSet::empty(),
        )));
        restore_tty_signals().unwrap();
    }
}
