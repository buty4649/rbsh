#[cfg(test)]
mod test {
    use super::*;
    use rbsh_parser::Location;

    macro_rules! word {
        ($e: expr) => {
            Word::new($e.to_string(), WordKind::Normal, Location::new(1, 1))
        };
    }

    macro_rules! wordlist {
        ($($e: expr$(,)?)+) => {
            vec![$($e,)+]
        };
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
        /*
        let mock = Wrapper::new();
        let mut e = Executor::new(mock).unwrap();
        assert_eq!(
            ExitStatus::new(0),
            e.execute_simple_command(vec![], RedirectList::new(), false, None, None, None)
        );

        /* parent */
        let mut mock = Wrapper::new();
        mock.expect_fork()
            .times(1)
            .return_const(Ok(ForkResult::Parent {
                child: Pid::from_raw(1000),
            }));
        mock.expect_setpgid()
            .times(1)
            .with(eq(Pid::from_raw(1000)), eq(Pid::from_raw(1000)))
            .return_const(Ok(()));
        mock.expect_tcgetsid()
            .times(1)
            .with(eq(0))
            .return_const(Ok(Pid::from_raw(900)));
        mock.expect_tcsetpgrp()
            .times(1)
            .with(eq(0), eq(Pid::from_raw(1000)))
            .return_const(Ok(()));
        mock.expect_waitpid()
            .times(1)
            .with(eq(Pid::from_raw(-1000)), eq(None))
            .return_const(Ok(WaitStatus::Exited(Pid::from_raw(1000), 0)));
        mock.expect_getpgid()
            .times(1)
            .with(eq(None))
            .return_const(Ok(Pid::from_raw(900)));
        mock.expect_tcgetsid()
            .times(1)
            .with(eq(0))
            .return_const(Ok(Pid::from_raw(1000)));
        mock.expect_tcsetpgrp()
            .times(1)
            .with(eq(0), eq(Pid::from_raw(900)))
            .return_const(Ok(()));
        let mut e = Executor::new(mock);
        assert_eq!(
            Ok(ExitStatus::new(0)),
            e.execute_command(UnitKind::SimpleCommand {
                command: vec![wordlist![word!["/foo/bar"]]],
                redirect: RedirectList::new(),
                background: false,
            })
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
        mock_restore_tty_signals(&mut mock);
        let mut e = Executor::new(mock).unwrap();
        assert_eq!(
            ExitStatus::new(127),
            e.execute_command(UnitKind::SimpleCommand {
                command: vec![wordlist![word!["/foo/bar"]]],
                redirect: RedirectList::new(),
                background: false,
            })
        );
        */
    }

    #[test]
    fn test_expand_command_line() {
        let ctx = Context::new();
        assert_eq!(
            Some(SimpleCommandKind::External {
                env: HashMap::new(),
                command: "foo".to_string(),
                args: vec![],
            }),
            expand_command_line(&ctx, vec![wordlist![word!("foo")]]).ok()
        );

        assert_eq!(
            Some(SimpleCommandKind::External {
                env: hashmap![("foo", "bar"), ("baz", "foo")],
                command: "bar".to_string(),
                args: Args::new()
            }),
            expand_command_line(
                &ctx,
                vec![
                    wordlist![word!("foo=bar")],
                    wordlist![word!("baz=foo")],
                    wordlist![word!("bar")]
                ]
            )
            .ok()
        );

        assert_eq!(
            Some(SimpleCommandKind::External {
                env: hashmap![("foo", "bar")],
                command: "baz".to_string(),
                args: vec!["hoge=fuga".to_string()],
            }),
            expand_command_line(
                &ctx,
                vec![
                    wordlist![word!("foo=bar")],
                    wordlist![word!("baz")],
                    wordlist![word!("hoge=fuga")]
                ]
            )
            .ok()
        );
    }
}
