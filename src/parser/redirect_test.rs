#[cfg(test)]
mod test {
    use super::super::word::WordKind;
    use super::super::ParseError;
    use super::*;
    use crate::*;

    macro_rules! got {
        ($f: ident) => {{
            |i: Vec<_>| $f(&mut i.into_iter().peekable())
        }};
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
        test_case! {
            got!(parse_redirect) => {
                // < foobar
                vec![
                    Token::read_from(loc!(1, 1)),
                    Token::space(loc!(2, 1)),
                    normal_word!("foobar", loc!(3, 1)),
                ] => ok![read_from,
                    0, wl![normal_word!("foobar", loc!(3, 1))], loc!(1, 1)
                ],

                // 123< foobar
                vec![
                    number!("123"),
                    Token::read_from(loc!(4, 1)),
                    Token::space(loc!(5, 1)),
                    normal_word!("foobar", loc!(6, 1)),
                ] => ok![read_from,
                    123, wl![normal_word!("foobar", loc!(6, 1))], loc!(1, 1)
                ],

                // 12345678901234567890 < foobar
                vec![
                    number!("12345678901234567890"),
                    Token::read_from(loc!(4, 1)),
                    Token::space(loc!(5, 1)),
                    normal_word!("foobar", loc!(6, 1)),
                ] => Err(ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))),
            },
        }
    }

    #[test]
    fn test_parse_redirect_writeto() {
        test_case! {
            got!(parse_redirect) => {
                // > foobar
                vec![
                    Token::write_to(loc!(1, 1)),
                    Token::space(loc!(2, 1)),
                    normal_word!("foobar", loc!(3, 1)),
                ] => ok![write_to,
                    1, wl![normal_word!("foobar", loc!(3, 1))], false, loc!(1, 1)
                ],

                // 123> foobar
                vec![
                    number!("123"),
                    Token::write_to(loc!(4, 1)),
                    Token::space(loc!(5, 1)),
                    normal_word!("foobar", loc!(6, 1)),
                ] => ok![write_to, 123, wl![normal_word!("foobar", loc!(6, 1))], false, loc!(1, 1)],

                // 123>| foobar
                vec![
                    number!("123"),
                    Token::force_write_to(loc!(4, 1)),
                    Token::space(loc!(6, 1)),
                    normal_word!("foobar", loc!(7, 1)),
                ] => ok![write_to, 123, wl![normal_word!("foobar", loc!(7, 1))], true, loc!(1, 1)],
            },
        }
    }

    #[test]
    fn test_close() {
        test_case! {
            got!(parse_redirect) => {
                // <&-
                vec![Token::read_close(loc!(1, 1))] => ok![close, 0, loc!(1, 1)],
                // >&-
                vec![Token::write_close(loc!(1, 1))] => ok![close, 1, loc!(1, 1)],
                // 123<&-
                vec![number!("123"), Token::read_close(loc!(1, 1))] => ok![close, 123, loc!(1, 1)],
                // 12345678901234567890<&-
                vec![
                    number!("12345678901234567890"),
                    Token::read_close(loc!(1, 1)),
                ] => Err(ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))),
                // 123>&-
                vec![number!("123"), Token::write_close(loc!(1, 1))] => ok![close, 123, loc!(1, 1)],
                // 12345678901234567890>&-
                vec![
                    number!("12345678901234567890"),
                    Token::write_close(loc!(1, 1)),
                ] => Err(ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))),
            },
        }
    }

    #[test]
    fn test_writeboth() {
        test_case! {
            got!(parse_redirect) => {
                // &> foobar
                vec![
                    Token::write_both(loc!(1, 1)),
                    Token::space(loc!(3, 1)),
                    normal_word!("foobar", loc!(4, 1)),
                ] => ok![write_both, wl![normal_word!("foobar", loc!(4, 1))], loc!(1, 1)],
                // >& foobar
                vec![
                    Token::write_both(loc!(1, 1)),
                    Token::space(loc!(3, 1)),
                    normal_word!("foobar", loc!(4, 1)),
                ] => ok![write_both, wl![normal_word!("foobar", loc!(4, 1))], loc!(1, 1)],
                // >&&
                vec![
                    Token::write_both(loc!(1, 1)),
                    Token::and(loc!(3, 1))
                ] => Err(ParseError::unexpected_token(Token::and(loc!(3, 1)))),
            },
        }
    }

    #[test]
    fn test_readcopy() {
        test_case! {
            got!(parse_redirect) => {
                // <&123
                vec![
                    Token::read_copy(loc!(1, 1)),
                    number!("123", loc!(3, 1)),
                    normal_word!("foobar", loc!(6, 1)),
                ] => ok![read_copy, 123, 0, false, loc!(1, 1)],
                // <&
                vec![Token::read_copy(loc!(1, 1))] => Err(ParseError::eof(loc!(3, 1))),
                // <& foobar
                vec![
                    Token::read_copy(loc!(1, 1)),
                    Token::space(loc!(3, 1)),
                    normal_word!("foobar", loc!(4, 1)),
                ] => Err(ParseError::unexpected_token(Token::space(loc!(3, 1)))),
                // <&12345678901234567890
                vec![
                    Token::read_copy(loc!(1, 1)),
                    number!("12345678901234567890", loc!(3, 1)),
                ] => Err(ParseError::invalid_fd("12345678901234567890".to_string(), loc!(3, 1))),
                // <&123-
                vec![
                    Token::read_copy(loc!(1, 1)),
                    number!("123", loc!(3, 1)),
                    Token::hyphen(loc!(4, 1)),
                ] => ok!(read_copy, 123, 0, true, loc!(1, 1)),
                // 123<&456
                vec![
                    number!("123"),
                    Token::read_copy(loc!(4, 1)),
                    number!("456", loc!(6, 1)),
                ] => ok![read_copy, 456, 123, false, loc!(1, 1)],
                // 123<&456-
                vec![
                    number!("123"),
                    Token::read_copy(loc!(4, 1)),
                    number!("456", loc!(6, 1)),
                    Token::hyphen(loc!(9, 1)),
                ] => ok![read_copy, 456, 123, true, loc!(1, 1)],
            },
        }
    }

    #[test]
    fn test_writecopy() {
        test_case! {
            got!(parse_redirect) => {
                // >&123
                vec![
                    Token::write_copy(loc!(1, 1)),
                    number!("123", loc!(3, 1)),
                    normal_word!("foobar", loc!(6, 1)),
                ] => ok![write_copy, 123, 1, false, loc!(1, 1)],
                vec![
                    Token::write_copy(loc!(1, 1)),
                    normal_word!("foobar", loc!(3, 1))
                ] => Err(ParseError::unexpected_token(normal_word!("foobar", loc!(3, 1)))),
                vec![
                    Token::write_copy(loc!(1, 1)),
                    number!("12345678901234567890", loc!(3, 1)),
                ] => Err(ParseError::invalid_fd("12345678901234567890".to_string(), loc!(3, 1))),

                // >&123-
                vec![
                    Token::write_copy(loc!(1, 1)),
                    number!("123", loc!(3, 1)),
                    Token::hyphen(loc!(4, 1)),
                ] => ok![write_copy, 123, 1, true, loc!(1, 1)],

                // 123>&456
                vec![
                    number!("123", loc!(1, 1)),
                    Token::write_copy(loc!(4, 1)),
                    number!("456", loc!(6, 1)),
                ] => ok![write_copy, 456, 123, false, loc!(1, 1)],

                // 123>&456-
                vec![
                    number!("123", loc!(1, 1)),
                    Token::write_copy(loc!(4, 1)),
                    number!("456", loc!(6, 1)),
                    Token::hyphen(loc!(9, 1)),
                ] => ok![write_copy, 456, 123, true, loc!(1, 1)],
            },
        }
    }

    #[test]
    fn test_append() {
        test_case! {
            got!(parse_redirect) => {
                // >> foobar
                vec![
                    Token::append(loc!(1, 1)),
                    Token::space(loc!(3, 1)),
                    normal_word!("foobar", loc!(4, 1)),
                ] => ok![append, 1, wl![normal_word!("foobar", loc!(4, 1))], loc!(1, 1)],

                // n>> foobar
                vec![
                    number!("123", loc!(1, 1)),
                    Token::append(loc!(2, 1)),
                    Token::space(loc!(4, 1)),
                    normal_word!("foobar", loc!(5, 1)),
                ] => ok![append, 123, wl![normal_word!("foobar", loc!(5, 1))], loc!(1, 1)],

                vec![
                    number!("12345678901234567890", loc!(1, 1)),
                    Token::append(loc!(2, 1)),
                    Token::space(loc!(4, 1)),
                    normal_word!("foobar", loc!(5, 1)),
                ] => Err(ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))),
            },
        }
    }

    #[test]
    fn test_append_both() {
        test_case! {
            got!(parse_redirect) => {
                // &>> foobar
                vec![
                    Token::append_both(loc!(1, 1)),
                    Token::space(loc!(4, 1)),
                    normal_word!("foobar", loc!(5, 1)),
                ] => ok![append_both, wl![normal_word!("foobar", loc!(5, 1))], loc!(1, 1)],

                vec![Token::append_both(loc!(1, 1)), Token::and(loc!(4, 1))]
                => Err(ParseError::unexpected_token(Token::and(loc!(4, 1)))),
            },
        }
    }

    #[test]
    fn test_readwrite() {
        test_case! {
            got!(parse_redirect) => {
                // <> foobar
                vec![
                    Token::read_write(loc!(1, 1)),
                    Token::space(loc!(3, 1)),
                    normal_word!("foobar", loc!(4, 1)),
                ] => ok![read_write, 0, wl![normal_word!("foobar", loc!(4, 1))], loc!(1,1)],

                // 123<> foobar
                vec![
                    number!("123"),
                    Token::read_write(loc!(2, 1)),
                    Token::space(loc!(4, 1)),
                    normal_word!("foobar", loc!(5, 1)),
                ] => ok![read_write, 123, wl![normal_word!("foobar", loc!(5, 1))], loc!(1, 1)],

                vec![Token::read_write(loc!(1, 1)), Token::and(loc!(3, 1))]
                => Err(ParseError::unexpected_token(Token::and(loc!(3, 1)))),

                vec![
                    number!("12345678901234567890"),
                    Token::read_write(loc!(2, 1)),
                    Token::space(loc!(4, 1)),
                    normal_word!("foobar", loc!(5, 1)),
                ] => Err(ParseError::invalid_fd("12345678901234567890".to_string(), loc!(1, 1))),
            },
        }
    }
}
