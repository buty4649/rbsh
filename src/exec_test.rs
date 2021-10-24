#[cfg(test)]
mod test {
    use super::syscall::{SysCallError, Wrapper};
    use super::*;
    use crate::location::Location;
    use mockall::predicate::{always, eq};
    use nix::{
        errno::Errno,
        sys::wait::WaitStatus,
        unistd::{ForkResult, Pid},
    };

    macro_rules! word {
        ($e: expr) => {
            Word::new($e.to_string(), WordKind::Normal, Location::new(1, 1))
        };
    }

    macro_rules! wordlist {
        ($($e: expr$(,)?)+) => {{
            let mut wl = WordList::new();
            $(wl.push($e);)+
            wl
        }};
    }

    macro_rules! hashmap {
        ($(($k: expr, $v: expr)$(,)?)+) => {{
            let mut h = HashMap::new();
            $(h.insert($k.to_string(), $v.to_string());)+
            h
        }};
    }

    #[test]
    fn test_simple_command() {
        let mock = Wrapper::new();
        let e = Executor::new(vec![]);
        let mut ctx = Context::new_at(mock);
        assert_eq!(
            Ok(ExitStatus::new(0)),
            e.execute_simple_command(&mut ctx, vec![], RedirectList::new(), false)
        );

        /* parent */
        let mut mock = Wrapper::new();
        mock.expect_fork()
            .times(1)
            .return_const(Ok(ForkResult::Parent {
                child: Pid::from_raw(1000),
            }));
        mock.expect_waitpid()
            .times(1)
            .with(eq(Pid::from_raw(1000)), eq(None))
            .return_const(Ok(WaitStatus::Exited(Pid::from_raw(1000), 0)));
        let e = Executor::new(vec![]);
        let mut ctx = Context::new_at(mock);
        assert_eq!(
            Ok(ExitStatus::new(0)),
            e.execute_command(
                &mut ctx,
                UnitKind::SimpleCommand {
                    command: vec![wordlist![word!["/foo/bar"]]],
                    redirect: RedirectList::new(),
                    background: false,
                }
            )
        );

        let mut mock = Wrapper::new();
        mock.expect_fork()
            .times(1)
            .return_const(Ok(ForkResult::Parent {
                child: Pid::from_raw(1000),
            }));
        mock.expect_waitpid()
            .times(1)
            .with(eq(Pid::from_raw(1000)), eq(None))
            .return_const(Err(SysCallError::new("waitpid", Errno::EINVAL)));
        let e = Executor::new(vec![]);
        let mut ctx = Context::new_at(mock);
        assert_eq!(
            Err(ShellError::syscall_error(
                SysCallError::new("waitpid", Errno::EINVAL),
                Location::new(1, 1)
            )),
            e.execute_command(
                &mut ctx,
                UnitKind::SimpleCommand {
                    command: vec![wordlist![word!["/foo/bar"]]],
                    redirect: RedirectList::new(),
                    background: false,
                }
            )
        );

        /* child */
        let mut mock = Wrapper::new();
        mock.expect_fork()
            .times(1)
            .return_const(Ok(ForkResult::Child));
        mock.expect_execve()
            .times(1)
            .with(
                eq("/foo/bar".to_cstring()),
                eq(vec!["/foo/bar".to_cstring()]),
                always(),
            )
            .return_const(Err(SysCallError::new("execve", Errno::ENOENT)));
        mock.expect_exit()
            .times(1)
            .with(eq(127))
            .return_const(ExitStatus::new(127));
        let e = Executor::new(vec![]);
        let mut ctx = Context::new_at(mock);
        assert_eq!(
            Ok(ExitStatus::new(127)),
            e.execute_command(
                &mut ctx,
                UnitKind::SimpleCommand {
                    command: vec![wordlist![word!["/foo/bar"]]],
                    redirect: RedirectList::new(),
                    background: false,
                }
            )
        );
    }

    #[test]
    fn test_split_env_and_commands() {
        let ctx = Context::new();
        assert_eq!(
            (HashMap::new(), vec!["foo".to_string()]),
            split_env_and_commands(&ctx, vec![wordlist![word!("foo")]])
        );

        assert_eq!(
            (
                hashmap![("foo", "bar"), ("baz", "foo")],
                vec!["bar".to_string()]
            ),
            split_env_and_commands(
                &ctx,
                vec![
                    wordlist![word!("foo=bar")],
                    wordlist![word!("baz=foo")],
                    wordlist![word!("bar")]
                ]
            )
        );

        assert_eq!(
            (
                hashmap![("foo", "bar")],
                vec!["baz".to_string(), "hoge=fuga".to_string()]
            ),
            split_env_and_commands(
                &ctx,
                vec![
                    wordlist![word!("foo=bar")],
                    wordlist![word!("baz")],
                    wordlist![word!("hoge=fuga")]
                ]
            )
        );
    }
}
