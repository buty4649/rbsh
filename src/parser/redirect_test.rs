#[cfg(test)]
mod test {
    use super::super::{ParseError, WordKind};
    use super::*;

    macro_rules! ok {
        ($i: ident, $s: expr) => {{
            $i(&mut $s.into_iter().peekable())
                .map_err(|t| t.value)
                .unwrap()
        }};
    }

    macro_rules! err {
        ($i: ident, $s: expr, $e: expr) => {{
            let got = $i(&mut $s.into_iter().peekable());
            assert_eq!(Err($e), got);
        }};
    }

    macro_rules! loc {
        ($c: expr, $l: expr) => {
            Location::new($c, $l)
        };
    }

    macro_rules! word {
        ($s: expr, $c: expr, $l: expr) => {
            Token::word($s.to_string(), WordKind::Normal, loc!($c, $l))
        };
    }

    macro_rules! number {
        ($s: expr, $c: expr, $l: expr) => {
            Token::number($s.to_string(), loc!($c, $l))
        };
    }

    macro_rules! space {
        ($c: expr, $l: expr) => {
            Token::space(loc!($c, $l))
        };
    }

    macro_rules! assert_redirect {
        ($f: ident, $t: expr, $e: expr, $c: expr, $l: expr) => {
            let got = ok!($f, $t);
            let expect = Token::redirect($e, loc!($c, $l));
            assert_eq!(Some(expect), got);
        };
    }

    #[test]
    fn test_readfrom() {
        // < foobar
        let tests = vec![
            Token::read_from(loc!(1, 1)),
            space!(2, 1),
            word!("foobar", 3, 1),
        ];
        let expect = RedirectKind::ReadFrom(0, vec![word!("foobar", 3, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // 123< foobar
        let tests = vec![
            number!("123", 1, 1),
            Token::read_from(loc!(4, 1)),
            space!(5, 1),
            word!("foobar", 6, 1),
        ];
        let expect = RedirectKind::ReadFrom(123, vec![word!("foobar", 6, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        let tests = vec![
            number!("12345678901234567890", 1, 1),
            Token::read_from(loc!(4, 1)),
            space!(5, 1),
            word!("foobar", 6, 1),
        ];
        err!(
            parse_redirect,
            tests,
            ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))
        );
    }

    #[test]
    fn test_parse_redirect_writeto() {
        // > foobar
        let tests = vec![
            Token::write_to(loc!(1, 1)),
            space!(2, 1),
            word!("foobar", 3, 1),
        ];
        let redirect = RedirectKind::WriteTo(1, vec![word!("foobar", 3, 1)], false);
        let expects = Token::redirect(redirect, Location::new(1, 1));
        let got = ok!(parse_redirect, tests);
        assert_eq!(Some(expects), got);

        // 123> foobar
        let tests = vec![
            Token::number("123".to_string(), loc!(1, 1)),
            Token::write_to(loc!(4, 1)),
            space!(5, 1),
            word!("foobar", 6, 1),
        ];
        let redirect = RedirectKind::WriteTo(123, vec![word!("foobar", 6, 1)], false);
        let expects = Token::redirect(redirect, Location::new(1, 1));
        let got = ok!(parse_redirect, tests);
        assert_eq!(Some(expects), got);

        // 123>| foobar
    }

    #[test]
    fn test_close() {
        // <&-
        let tests = vec![Token::read_close(loc!(1, 1))];
        let expect = RedirectKind::Close(0);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // >&-
        let tests = vec![Token::write_close(loc!(1, 1))];
        let expect = RedirectKind::Close(1);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // 123<&-
        {
            let tests = vec![number!("123", 1, 1), Token::read_close(loc!(1, 1))];
            let expect = RedirectKind::Close(123);
            assert_redirect!(parse_redirect, tests, expect, 1, 1);

            let tests = vec![
                number!("12345678901234567890", 1, 1),
                Token::read_close(loc!(1, 1)),
            ];
            err!(
                parse_redirect,
                tests,
                ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))
            );
        }

        // 123>&-
        {
            let tests = vec![number!("123", 1, 1), Token::write_close(loc!(1, 1))];
            let expect = RedirectKind::Close(123);
            assert_redirect!(parse_redirect, tests, expect, 1, 1);

            let tests = vec![
                number!("12345678901234567890", 1, 1),
                Token::write_close(loc!(1, 1)),
            ];
            err!(
                parse_redirect,
                tests,
                ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))
            );
        }
    }

    #[test]
    fn test_writeboth() {
        // &> foobar
        let tests = vec![
            Token::write_both(loc!(1, 1)),
            space!(3, 1),
            word!("foobar", 4, 1),
        ];
        let expect = RedirectKind::WriteBoth(vec![word!("foobar", 4, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // >& foobar
        let tests = vec![
            Token::write_both(loc!(1, 1)),
            space!(3, 1),
            word!("foobar", 4, 1),
        ];
        let expect = RedirectKind::WriteBoth(vec![word!("foobar", 4, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        let tests = vec![Token::write_both(loc!(1, 1)), Token::and(loc!(3, 1))];
        err!(
            parse_redirect,
            tests,
            ParseError::unexpected_token(Token::and(loc!(3, 1)))
        );
    }

    #[test]
    fn test_readcopy() {
        // <&123
        {
            let tests = vec![
                Token::read_copy(loc!(1, 1)),
                number!("123", 3, 1),
                word!("foobar", 6, 1),
            ];
            let expect = RedirectKind::ReadCopy(123, 0, false);
            assert_redirect!(parse_redirect, tests, expect, 1, 1);

            let tests = vec![Token::read_copy(loc!(1, 1))];
            err!(parse_redirect, tests, ParseError::eof(loc!(3, 1)));

            let tests = vec![
                Token::read_copy(loc!(1, 1)),
                space!(3, 1),
                word!("foobar", 4, 1),
            ];
            err!(
                parse_redirect,
                tests,
                ParseError::unexpected_token(Token::space(loc!(3, 1)))
            );

            let tests = vec![
                Token::read_copy(loc!(1, 1)),
                number!("12345678901234567890", 3, 1),
            ];
            err!(
                parse_redirect,
                tests,
                ParseError::invalid_fd("12345678901234567890".to_string(), loc!(3, 1))
            );
        }

        // <&123-
        let tests = vec![
            Token::read_copy(loc!(1, 1)),
            number!("123", 3, 1),
            Token::hyphen(loc!(4, 1)),
        ];
        let expect = RedirectKind::ReadCopy(123, 0, true);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // 123<&456
        let tests = vec![
            number!("123", 1, 1),
            Token::read_copy(loc!(4, 1)),
            number!("456", 6, 1),
        ];
        let expect = RedirectKind::ReadCopy(456, 123, false);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // 123<&456-
        let tests = vec![
            number!("123", 1, 1),
            Token::read_copy(loc!(4, 1)),
            number!("456", 6, 1),
            Token::hyphen(loc!(9, 1)),
        ];
        let expect = RedirectKind::ReadCopy(456, 123, true);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);
    }

    #[test]
    fn test_writecopy() {
        // >&123
        {
            let tests = vec![
                Token::write_copy(loc!(1, 1)),
                number!("123", 3, 1),
                word!("foobar", 6, 1),
            ];
            let expect = RedirectKind::WriteCopy(123, 1, false);
            assert_redirect!(parse_redirect, tests, expect, 1, 1);

            let tests = vec![Token::write_copy(loc!(1, 1)), word!("foobar", 3, 1)];
            err!(
                parse_redirect,
                tests,
                ParseError::unexpected_token(Token::word(
                    "foobar".to_string(),
                    WordKind::Normal,
                    loc!(3, 1)
                ))
            );

            let tests = vec![
                Token::write_copy(loc!(1, 1)),
                number!("12345678901234567890", 3, 1),
            ];
            err!(
                parse_redirect,
                tests,
                ParseError::invalid_fd("12345678901234567890".to_string(), loc!(3, 1))
            );
        }

        // >&123-
        let tests = vec![
            Token::write_copy(loc!(1, 1)),
            number!("123", 3, 1),
            Token::hyphen(loc!(4, 1)),
        ];
        let expect = RedirectKind::WriteCopy(123, 1, true);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // 123>&456
        let tests = vec![
            number!("123", 1, 1),
            Token::write_copy(loc!(4, 1)),
            number!("456", 6, 1),
        ];
        let expect = RedirectKind::WriteCopy(456, 123, false);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // 123>&456-
        let tests = vec![
            number!("123", 1, 1),
            Token::write_copy(loc!(4, 1)),
            number!("456", 6, 1),
            Token::hyphen(loc!(9, 1)),
        ];
        let expect = RedirectKind::WriteCopy(456, 123, true);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);
    }

    #[test]
    fn test_append() {
        // >> foobar
        let tests = vec![
            Token::append(loc!(1, 1)),
            space!(3, 1),
            word!("foobar", 4, 1),
        ];
        let expect = RedirectKind::Append(1, vec![word!("foobar", 4, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // n>> foobar
        let tests = vec![
            number!("123", 1, 1),
            Token::append(loc!(2, 1)),
            space!(4, 1),
            word!("foobar", 5, 1),
        ];
        let expect = RedirectKind::Append(123, vec![word!("foobar", 5, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        let tests = vec![
            number!("12345678901234567890", 1, 1),
            Token::append(loc!(2, 1)),
            space!(4, 1),
            word!("foobar", 5, 1),
        ];
        err!(
            parse_redirect,
            tests,
            ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))
        );
    }

    #[test]
    fn test_append_both() {
        // &>> foobar
        let tests = vec![
            Token::append_both(loc!(1, 1)),
            space!(4, 1),
            word!("foobar", 5, 1),
        ];
        let expect = RedirectKind::AppendBoth(vec![word!("foobar", 5, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        let tests = vec![Token::append_both(loc!(1, 1)), Token::and(loc!(4, 1))];
        err!(
            parse_redirect,
            tests,
            ParseError::unexpected_token(Token::and(loc!(4, 1)))
        );
    }

    #[test]
    fn test_readwrite() {
        // <> foobar
        let tests = vec![
            Token::read_write(loc!(1, 1)),
            space!(3, 1),
            word!("foobar", 4, 1),
        ];
        let expect = RedirectKind::ReadWrite(0, vec![word!("foobar", 4, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        // 123<> foobar
        let tests = vec![
            number!("123", 1, 1),
            Token::read_write(loc!(2, 1)),
            space!(4, 1),
            word!("foobar", 5, 1),
        ];
        let expect = RedirectKind::ReadWrite(123, vec![word!("foobar", 5, 1)]);
        assert_redirect!(parse_redirect, tests, expect, 1, 1);

        let tests = vec![Token::read_write(loc!(1, 1)), Token::and(loc!(3, 1))];
        err!(
            parse_redirect,
            tests,
            ParseError::unexpected_token(Token::and(loc!(3, 1)))
        );

        let tests = vec![
            number!("12345678901234567890", 1, 1),
            Token::read_write(loc!(2, 1)),
            space!(4, 1),
            word!("foobar", 5, 1),
        ];
        err!(
            parse_redirect,
            tests,
            ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))
        );
    }
}
