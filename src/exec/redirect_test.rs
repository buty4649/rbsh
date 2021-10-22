#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        parser::{
            redirect::Redirect,
            word::{Word, WordKind},
        },
        Location,
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
        ($m: expr ) => {
            RedirectApplier::new($m)
        };
    }

    #[test]
    fn test_read_from() {
        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_RDONLY),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        r!(mock)
            .exec(vec![Redirect::read_from(
                3,
                wordlist![word!("foobar")],
                Location::new(1, 1),
            )])
            .unwrap();

        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_RDONLY),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        mock.expect_dup2()
            .times(1)
            .with(eq(3), eq(0))
            .return_const(Ok(0));
        mock.expect_close()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        r!(mock)
            .exec(vec![Redirect::read_from(
                0,
                wordlist![word!("foobar")],
                Location::new(1, 1),
            )])
            .unwrap();
    }

    #[test]
    fn test_write_to() {
        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        r!(mock)
            .exec(vec![Redirect::write_to(
                3,
                wordlist![word!("foobar")],
                false,
                Location::new(1, 1),
            )])
            .unwrap();

        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        mock.expect_dup2()
            .times(1)
            .with(eq(3), eq(1))
            .return_const(Ok(1));
        mock.expect_close()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        r!(mock)
            .exec(vec![Redirect::write_to(
                1,
                wordlist![word!("foobar")],
                false,
                Location::new(1, 1),
            )])
            .unwrap();
    }

    #[test]
    fn test_write_both() {
        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        mock.expect_dup2()
            .times(1)
            .with(eq(3), eq(1))
            .return_const(Ok(1));
        mock.expect_dup2()
            .times(1)
            .with(eq(1), eq(2))
            .return_const(Ok(2));
        mock.expect_close()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        r!(mock)
            .exec(vec![Redirect::write_both(
                wordlist![word!("foobar")],
                Location::new(1, 1),
            )])
            .unwrap();
    }

    #[test]
    fn test_copy() {
        let mut mock = Wrapper::new();
        mock.expect_dup2()
            .times(1)
            .with(eq(3), eq(4))
            .return_const(Ok(4));
        r!(mock)
            .exec(vec![Redirect::copy(3, 4, false, Location::new(1, 1))])
            .unwrap();

        let mut mock = Wrapper::new();
        mock.expect_dup2()
            .times(1)
            .with(eq(3), eq(4))
            .return_const(Ok(4));
        mock.expect_close()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        r!(mock)
            .exec(vec![Redirect::copy(3, 4, true, Location::new(1, 1))])
            .unwrap();
    }

    #[test]
    fn test_append() {
        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        r!(mock)
            .exec(vec![Redirect::append(
                3,
                wordlist![word!("foobar")],
                Location::new(1, 1),
            )])
            .unwrap();

        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        mock.expect_dup2()
            .times(1)
            .with(eq(3), eq(1))
            .return_const(Ok(1));
        mock.expect_close()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        r!(mock)
            .exec(vec![Redirect::append(
                1,
                wordlist![word!("foobar")],
                Location::new(1, 1),
            )])
            .unwrap();
    }

    #[test]
    fn test_append_both() {
        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        mock.expect_dup2()
            .times(1)
            .with(eq(3), eq(1))
            .return_const(Ok(1));
        mock.expect_dup2()
            .times(1)
            .with(eq(1), eq(2))
            .return_const(Ok(2));
        mock.expect_close()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        r!(mock)
            .exec(vec![Redirect::append_both(
                wordlist![word!("foobar")],
                Location::new(1, 1),
            )])
            .unwrap();
    }

    #[test]
    fn test_close() {
        let mut mock = Wrapper::new();
        mock.expect_close()
            .times(1)
            .with(eq(1))
            .return_const(Ok(()));
        r!(mock)
            .exec(vec![Redirect::close(1, Location::new(1, 1))])
            .unwrap();
    }

    #[test]
    fn test_read_write() {
        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_RDWR | OFlag::O_CREAT),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        r!(mock)
            .exec(vec![Redirect::read_write(
                3,
                wordlist![word!("foobar")],
                Location::new(1, 1),
            )])
            .unwrap();

        let mut mock = Wrapper::new();
        mock.expect_open()
            .times(1)
            .with(
                eq("foobar"),
                eq(OFlag::O_RDWR | OFlag::O_CREAT),
                eq(Mode::from_bits(0o666).unwrap()),
            )
            .return_const(Ok(3));
        mock.expect_dup2()
            .times(1)
            .with(eq(3), eq(0))
            .return_const(Ok(0));
        mock.expect_close()
            .times(1)
            .with(eq(3))
            .return_const(Ok(()));
        r!(mock)
            .exec(vec![Redirect::read_write(
                0,
                wordlist![word!("foobar")],
                Location::new(1, 1),
            )])
            .unwrap();
    }
}
