#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        loc, normal_word,
        parser::{lexer::lex, token::Token, word::WordKind, ParseError},
    };

    macro_rules! lex {
        ($e: expr) => {
            lex($e).unwrap()
        };
    }

    macro_rules! assert_redirect {
        ($e: expr, $expect: expr) => {
            let mut t = TokenReader::new(lex!($e));
            let got = parse_redirect(&mut t);
            assert_eq!($expect, got)
        };
    }

    macro_rules! ok {
        ($i: ident, $($a: expr$(,)?)+) => {
            Ok(Some(Redirect::$i($($a,)*)))
        };
    }

    macro_rules! wl {
        ($($w: expr$(,)?)+) => {{
            let mut list = WordList::new();
            $(list.push_word_token($w);)+
            list
        }};
    }

    #[test]
    fn test_readfrom() {
        assert_redirect!(
            "< foobar",
            ok![
                read_from,
                0,
                wl![normal_word!("foobar", loc!(3, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "123< foobar",
            ok![
                read_from,
                123,
                wl![normal_word!("foobar", loc!(6, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "12345678901234567890< foobar",
            Err(ParseError::invalid_fd("12345678901234567890", loc!(1, 1)))
        );
    }

    #[test]
    fn test_parse_redirect_writeto() {
        assert_redirect!(
            "> foobar",
            ok![
                write_to,
                1,
                wl![normal_word!("foobar", loc!(3, 1))],
                false,
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "123> foobar",
            ok![
                write_to,
                123,
                wl![normal_word!("foobar", loc!(6, 1))],
                false,
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "123>| foobar",
            ok![
                write_to,
                123,
                wl![normal_word!("foobar", loc!(7, 1))],
                true,
                loc!(1, 1)
            ]
        );
    }

    #[test]
    fn test_close() {
        assert_redirect!("<&-", ok![close, 0, loc!(1, 1)]);
        assert_redirect!(">&-", ok![close, 1, loc!(1, 1)]);
        assert_redirect!("123<&-", ok![close, 123, loc!(1, 1)]);
        assert_redirect!("123>&-", ok![close, 123, loc!(1, 1)]);

        assert_redirect!(
            "12345678901234567890<&-",
            Err(ParseError::invalid_fd("12345678901234567890", loc!(1, 1)))
        );

        assert_redirect!(
            "12345678901234567890>&-",
            Err(ParseError::invalid_fd("12345678901234567890", loc!(1, 1)))
        );
    }

    #[test]
    fn test_writeboth() {
        assert_redirect!(
            "&> foobar",
            ok![
                write_both,
                wl![normal_word!("foobar", loc!(4, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            ">& foobar",
            ok![
                write_both,
                wl![normal_word!("foobar", loc!(4, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            ">&&",
            Err(ParseError::unexpected_token(Token::background(loc!(3, 1))))
        );
    }

    #[test]
    fn test_readcopy() {
        assert_redirect!("<&123", ok![copy, 123, 0, false, loc!(1, 1)]);
        assert_redirect!("<&", Err(ParseError::eof(loc!(2, 1))));
        assert_redirect!("<&123-", ok!(copy, 123, 0, true, loc!(1, 1)));
        assert_redirect!("123<&456", ok![copy, 456, 123, false, loc!(1, 1)]);
        assert_redirect!("123<&456-", ok![copy, 456, 123, true, loc!(1, 1)]);

        assert_redirect!(
            "<& foobar",
            Err(ParseError::unexpected_token(Token::space(loc!(3, 1))))
        );

        assert_redirect!(
            "<&12345678901234567890",
            Err(ParseError::invalid_fd("12345678901234567890", loc!(3, 1)))
        );
    }

    #[test]
    fn test_writecopy() {
        assert_redirect!(">&123", ok![copy, 123, 1, false, loc!(1, 1)]);
        assert_redirect!(">&123-", ok![copy, 123, 1, true, loc!(1, 1)]);
        assert_redirect!("123>&456", ok![copy, 456, 123, false, loc!(1, 1)]);
        assert_redirect!("123>&456-", ok![copy, 456, 123, true, loc!(1, 1)]);

        assert_redirect!(
            "123>&foobar",
            Err(ParseError::unexpected_token(normal_word!(
                "foobar",
                loc!(6, 1)
            )))
        );

        assert_redirect!(
            ">&12345678901234567890",
            Err(ParseError::invalid_fd("12345678901234567890", loc!(3, 1)))
        );
    }

    #[test]
    fn test_append() {
        assert_redirect!(
            ">> foobar",
            ok![
                append,
                1,
                wl![normal_word!("foobar", loc!(4, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "123>> foobar",
            ok![
                append,
                123,
                wl![normal_word!("foobar", loc!(7, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "12345678901234567890>> foobar",
            Err(ParseError::invalid_fd("12345678901234567890", loc!(1, 1)))
        );
    }

    #[test]
    fn test_append_both() {
        assert_redirect!(
            "&>> foobar",
            ok![
                append_both,
                wl![normal_word!("foobar", loc!(5, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "&>>&",
            Err(ParseError::unexpected_token(Token::background(loc!(4, 1))))
        );
    }

    #[test]
    fn test_readwrite() {
        assert_redirect!(
            "<> foobar",
            ok![
                read_write,
                0,
                wl![normal_word!("foobar", loc!(4, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "123<> foobar",
            ok![
                read_write,
                123,
                wl![normal_word!("foobar", loc!(7, 1))],
                loc!(1, 1)
            ]
        );

        assert_redirect!(
            "<>&",
            Err(ParseError::unexpected_token(Token::background(loc!(3, 1))))
        );

        assert_redirect!(
            "12345678901234567890<> foobar",
            Err(ParseError::invalid_fd("12345678901234567890", loc!(1, 1)))
        );
    }
}
