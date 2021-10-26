use super::{
    context::Context,
    exec::syscall::{SysCallResult, SysCallWrapper},
};
use nix::sys::signal::{SaFlags, SigAction, SigHandler, SigSet, Signal};

static TTYSIGNALS: [Signal; 5] = [
    Signal::SIGQUIT,
    Signal::SIGTERM,
    Signal::SIGTSTP,
    Signal::SIGTTIN,
    Signal::SIGTTOU,
];

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
        ctx.wrapper().sigaction(sig, &sa)?;
    }
    Ok(())
}

pub fn restore_tty_signals(ctx: &Context) -> SysCallResult<()> {
    let sa = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
    for sig in TTYSIGNALS {
        ctx.wrapper().sigaction(sig, &sa)?;
    }
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::exec::syscall::Wrapper;
    use mockall::predicate::{eq, function};

    pub fn mock_ignore_tty_signals(mock: &mut Wrapper) {
        for sig in TTYSIGNALS {
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
        let ctx = Context::new_at(mock);
        ignore_tty_signals(&ctx).unwrap();
    }

    #[test]
    fn test_reset_tty_signals() {
        let mut mock = Wrapper::new();
        mock_restore_tty_signals(&mut mock);
        let ctx = Context::new_at(mock);
        restore_tty_signals(&ctx).unwrap();
    }
}
