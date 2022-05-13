#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        location::Location,
        parser::{
            redirect::Redirect,
            word::{Word, WordKind},
        },
    };
    use mockall::predicate::eq;

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

    macro_rules! r {
        ($b: expr) => {
            RedirectApplier::new($b)
        };
    }

    #[test]
    fn test_read_from() {
        let open_context = syscall::open_context();
        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_RDONLY),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::read_from(
                    3,
                    wordlist![word!("foobar")],
                    Location::new(1, 1),
                )],
            )
            .is_ok());

        open_context.checkpoint();
        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_RDONLY),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));

        let dup2_context = syscall::dup2_context();
        dup2_context
            .expect()
            .times(1)
            .with(eq(3), eq(0))
            .return_const(Ok(0));

        let close_context = syscall::close_context();
        close_context
            .expect()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));

        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::read_from(
                    0,
                    wordlist![word!("foobar")],
                    Location::new(1, 1),
                )],
            )
            .is_ok());
    }

    #[test]
    fn test_write_to() {
        let open_context = syscall::open_context();
        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::write_to(
                    3,
                    wordlist![word!("foobar")],
                    false,
                    Location::new(1, 1),
                )],
            )
            .is_ok());
        open_context.checkpoint();

        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));

        let dup2_context = syscall::dup2_context();
        dup2_context
            .expect()
            .times(1)
            .with(eq(3), eq(1))
            .return_const(Ok(1));

        let close_context = syscall::close_context();
        close_context
            .expect()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::write_to(
                    1,
                    wordlist![word!("foobar")],
                    false,
                    Location::new(1, 1),
                )],
            )
            .is_ok());
    }

    #[test]
    fn test_write_both() {
        let open_context = syscall::open_context();
        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));

        let dup2_context = syscall::dup2_context();
        dup2_context
            .expect()
            .times(1)
            .with(eq(3), eq(1))
            .return_const(Ok(1));
        dup2_context
            .expect()
            .times(1)
            .with(eq(1), eq(2))
            .return_const(Ok(2));

        let close_context = syscall::close_context();
        close_context
            .expect()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));

        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::write_both(
                    wordlist![word!("foobar")],
                    Location::new(1, 1),
                )],
            )
            .is_ok());
    }

    #[test]
    fn test_copy() {
        let dup2_context = syscall::dup2_context();
        dup2_context
            .expect()
            .times(1)
            .with(eq(3), eq(4))
            .return_const(Ok(4));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::copy(3, 4, false, Location::new(1, 1))],
            )
            .is_ok());
        dup2_context.checkpoint();

        dup2_context
            .expect()
            .times(1)
            .with(eq(3), eq(4))
            .return_const(Ok(4));

        let close_context = syscall::close_context();
        close_context
            .expect()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::copy(3, 4, true, Location::new(1, 1))],
            )
            .is_ok());
    }

    #[test]
    fn test_append() {
        let open_context = syscall::open_context();
        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::append(
                    3,
                    wordlist![word!("foobar")],
                    Location::new(1, 1),
                )],
            )
            .is_ok());
        open_context.checkpoint();

        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));

        let dup2_context = syscall::dup2_context();
        dup2_context
            .expect()
            .times(1)
            .with(eq(3), eq(1))
            .return_const(Ok(1));

        let close_context = syscall::close_context();
        close_context
            .expect()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));

        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::append(
                    1,
                    wordlist![word!("foobar")],
                    Location::new(1, 1),
                )],
            )
            .is_ok());
    }

    #[test]
    fn test_append_both() {
        let open_context = syscall::open_context();
        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));

        let dup2_context = syscall::dup2_context();
        dup2_context
            .expect()
            .times(1)
            .with(eq(3), eq(1))
            .return_const(Ok(1));
        dup2_context
            .expect()
            .times(1)
            .with(eq(1), eq(2))
            .return_const(Ok(2));

        let close_context = syscall::close_context();
        close_context
            .expect()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::append_both(
                    wordlist![word!("foobar")],
                    Location::new(1, 1),
                )],
            )
            .is_ok());
    }

    #[test]
    fn test_close() {
        let close_context = syscall::close_context();
        close_context
            .expect()
            .times(1)
            .with(eq(1))
            .return_const(Ok(()));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::close(1, Location::new(1, 1))]
            )
            .is_ok());
    }

    #[test]
    fn test_read_write() {
        let open_context = syscall::open_context();
        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_RDWR | OFlag::O_CREAT),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::read_write(
                    3,
                    wordlist![word!("foobar")],
                    Location::new(1, 1),
                )],
            )
            .is_ok());
        open_context.checkpoint();

        open_context
            .expect()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_RDWR | OFlag::O_CREAT),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));

        let dup2_context = syscall::dup2_context();
        dup2_context
            .expect()
            .times(1)
            .with(eq(3), eq(0))
            .return_const(Ok(0));

        let close_context = syscall::close_context();
        close_context
            .expect()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        assert!(r!(false)
            .exec(
                &Context::new(),
                vec![Redirect::read_write(
                    0,
                    wordlist![word!("foobar")],
                    Location::new(1, 1),
                )],
            )
            .is_ok());
    }
}
