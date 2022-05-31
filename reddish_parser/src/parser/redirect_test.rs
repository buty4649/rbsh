#[cfg(test)]
mod test {
    use super::*;
    use crate::{error::Error, lexer::Lexer, location, Token, WordKind};

    macro_rules! lex {
        ($e: expr) => {
            Lexer::new($e, 0).iter()
        };
    }

    macro_rules! assert_redirect {
        ($e: expr, $expect: expr) => {
            let got = parse_redirect(&mut lex!($e));
            assert_eq!(got, $expect)
        };
    }

    macro_rules! ok {
        ($i: ident, $($a: expr$(,)?)+) => {
            Ok(Some(vec![Redirect::$i($($a,)*)]))
        };
    }

    #[test]
    fn test_readfrom() {
        assert_redirect!(
            "< foobar",
            ok![
                read_from,
                0,
                vec![Word::normal("foobar", location!(3, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123< foobar",
            ok![
                read_from,
                123,
                vec![Word::normal("foobar", location!(6, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "12345678901234567890< foobar",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );
    }

    #[test]
    fn test_parse_redirect_writeto() {
        assert_redirect!(
            "> foobar",
            ok![
                write_to,
                1,
                vec![Word::normal("foobar", location!(3, 1))],
                false,
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123> foobar",
            ok![
                write_to,
                123,
                vec![Word::normal("foobar", location!(6, 1))],
                false,
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123>| foobar",
            ok![
                write_to,
                123,
                vec![Word::normal("foobar", location!(7, 1))],
                true,
                location!(1, 1)
            ]
        );
    }

    #[test]
    fn test_close() {
        assert_redirect!("<&-", ok![close, 0, location!(1, 1)]);
        assert_redirect!(">&-", ok![close, 1, location!(1, 1)]);
        assert_redirect!("123<&-", ok![close, 123, location!(1, 1)]);
        assert_redirect!("123>&-", ok![close, 123, location!(1, 1)]);

        assert_redirect!(
            "12345678901234567890<&-",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );

        assert_redirect!(
            "12345678901234567890>&-",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );
    }

    #[test]
    fn test_writeboth() {
        assert_redirect!(
            "&> foobar",
            ok![
                write_both,
                vec![Word::normal("foobar", location!(4, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            ">& foobar",
            ok![
                write_both,
                vec![Word::normal("foobar", location!(4, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            ">&&",
            Err(Error::unexpected_token(&Token::background(location!(3, 1))))
        );
    }

    #[test]
    fn test_readcopy() {
        assert_redirect!("<&123", ok![copy, 123, 0, false, location!(1, 1)]);
        assert_redirect!("<&", Err(Error::eof(location!(3, 1))));
        assert_redirect!("<&123-", ok!(copy, 123, 0, true, location!(1, 1)));
        assert_redirect!("123<&456", ok![copy, 456, 123, false, location!(1, 1)]);
        assert_redirect!("123<&456-", ok![copy, 456, 123, true, location!(1, 1)]);

        assert_redirect!(
            "<& foobar",
            Err(Error::unexpected_token(&Token::space(location!(3, 1))))
        );

        assert_redirect!(
            "<&12345678901234567890",
            Err(Error::invalid_fd("12345678901234567890", location!(3, 1)))
        );
    }

    #[test]
    fn test_writecopy() {
        assert_redirect!(">&123", ok![copy, 123, 1, false, location!(1, 1)]);
        assert_redirect!(">&123-", ok![copy, 123, 1, true, location!(1, 1)]);
        assert_redirect!("123>&456", ok![copy, 456, 123, false, location!(1, 1)]);
        assert_redirect!("123>&456-", ok![copy, 456, 123, true, location!(1, 1)]);

        assert_redirect!(
            "123>&foobar",
            Err(Error::unexpected_token(&Token::word(
                "foobar",
                WordKind::Normal,
                location!(6, 1)
            )))
        );

        assert_redirect!(
            ">&12345678901234567890",
            Err(Error::invalid_fd("12345678901234567890", location!(3, 1)))
        );
    }

    #[test]
    fn test_append() {
        assert_redirect!(
            ">> foobar",
            ok![
                append,
                1,
                vec![Word::normal("foobar", location!(4, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123>> foobar",
            ok![
                append,
                123,
                vec![Word::normal("foobar", location!(7, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "12345678901234567890>> foobar",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );
    }

    #[test]
    fn test_append_both() {
        assert_redirect!(
            "&>> foobar",
            ok![
                append_both,
                vec![Word::normal("foobar", location!(5, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "&>>&",
            Err(Error::unexpected_token(&Token::background(location!(4, 1))))
        );
    }

    #[test]
    fn test_readwrite() {
        assert_redirect!(
            "<> foobar",
            ok![
                read_write,
                0,
                vec![Word::normal("foobar", location!(4, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123<> foobar",
            ok![
                read_write,
                123,
                vec![Word::normal("foobar", location!(7, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "<>&",
            Err(Error::unexpected_token(&Token::background(location!(3, 1))))
        );

        assert_redirect!(
            "12345678901234567890<> foobar",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );
    }
}
