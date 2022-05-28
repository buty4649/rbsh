#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        cmd, error::Error, literal_word, loc, normal_word, number, param, quote_word, var,
    };

    macro_rules! assert_lex {
        ($f: ident, $s: expr, $expect: expr) => {{
            let mut lexer = Lexer::new($s, 1, false);
            let got = lexer.$f().map(|t| (t, lexer.location()));
            assert_eq!($expect, got)
        }};
        ($s: expr, $expect: expr) => {{
            assert_lex![lex, $s, $expect]
        }};
    }

    macro_rules! ok {
        ($e: expr, $l: expr) => {
            Ok(($e, $l))
        };
    }

    #[test]
    fn test_lex_keyword() {
        assert_lex!("if", ok![vec![Token::if_keyword(loc!(1, 1))], loc!(3, 1)]);

        assert_lex!(
            "then",
            ok![vec![Token::then_keyword(loc!(1, 1))], loc!(5, 1)]
        );

        assert_lex!("fi", ok![vec![Token::fi_keyword(loc!(1, 1))], loc!(3, 1)]);

        assert_lex!(
            "elif",
            ok![vec![Token::elif_keyword(loc!(1, 1))], loc!(5, 1)]
        );

        assert_lex!(
            "elsif",
            ok![vec![Token::elsif_keyword(loc!(1, 1))], loc!(6, 1)]
        );

        assert_lex!("end", ok![vec![Token::end_keyword(loc!(1, 1))], loc!(4, 1)]);

        assert_lex!(
            "unless",
            ok![vec![Token::unless_keyword(loc!(1, 1))], loc!(7, 1)]
        );

        assert_lex!(
            "while",
            ok![vec![Token::while_keyword(loc!(1, 1))], loc!(6, 1)]
        );

        assert_lex!("do", ok![vec![Token::do_keyword(loc!(1, 1))], loc!(3, 1)]);

        assert_lex!(
            "done",
            ok![vec![Token::done_keyword(loc!(1, 1))], loc!(5, 1)]
        );

        assert_lex!(
            "until",
            ok![vec![Token::until_keyword(loc!(1, 1))], loc!(6, 1)]
        );

        assert_lex!("for", ok![vec![Token::for_keyword(loc!(1, 1))], loc!(4, 1)]);

        assert_lex!(
            "echo if",
            ok![
                vec![
                    normal_word!("echo"),
                    Token::space(loc!(5, 1)),
                    normal_word!("if", loc!(6, 1)),
                ],
                loc!(8, 1)
            ]
        );

        assert_lex!(
            "for a in",
            ok![
                vec![
                    Token::for_keyword(loc!(1, 1)),
                    Token::space(loc!(4, 1)),
                    normal_word!("a", loc!(5, 1)),
                    Token::space(loc!(6, 1)),
                    Token::in_keyword(loc!(7, 1))
                ],
                loc!(9, 1)
            ]
        );
        assert_lex!("in", ok![vec![normal_word!("in")], loc!(3, 1)]);
    }

    #[test]
    fn test_lex_space() {
        assert_lex!(
            lex_space,
            "  \t  ",
            ok![Token::space(loc!(1, 1)), loc!(6, 1)]
        );
    }

    #[test]
    fn test_lex_newline() {
        assert_lex!(
            lex_newline,
            "\n\n\n \t",
            ok![Token::newline(loc!(1, 1)), loc!(1, 4)]
        );
    }

    #[test]
    fn test_lex_semicolon() {
        assert_lex!(
            lex_semicolon,
            ";;;\n\t",
            ok![Token::termination(loc!(1, 1)), loc!(4, 1)]
        );
    }

    #[test]
    fn test_lex_ampersand() {
        assert_lex!(
            lex_ampersand,
            "&",
            ok![Token::background(loc!(1, 1)), loc!(2, 1)]
        );
        assert_lex!(lex_ampersand, "&&", ok![Token::and(loc!(1, 1)), loc!(3, 1)]);
        assert_lex!(
            lex_ampersand,
            "&>",
            ok![Token::write_both(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_ampersand,
            "&>>",
            ok![Token::append_both(loc!(1, 1)), loc!(4, 1)]
        );
    }

    #[test]
    fn test_lex_hyphen() {
        assert_lex!(lex_hyphen, "-", ok![normal_word!("-"), loc!(2, 1)]);

        assert_lex!(
            "<&123-",
            ok![
                vec![
                    Token::read_copy(loc!(1, 1)),
                    number!("123", loc!(3, 1)),
                    Token::hyphen(loc!(6, 1))
                ],
                loc!(7, 1)
            ]
        );
    }

    #[test]
    fn test_lex_vertical_line() {
        assert_lex!(
            lex_vertical_line,
            "|",
            ok![Token::pipe(loc!(1, 1)), loc!(2, 1)]
        );
        assert_lex!(
            lex_vertical_line,
            "||",
            ok![Token::or(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_vertical_line,
            "|&",
            ok![Token::pipe_both(loc!(1, 1)), loc!(3, 1)]
        );
    }

    #[test]
    fn test_lex_number() {
        assert_lex!(lex_number, "123", ok![normal_word!("123"), loc!(4, 1)]);
        assert_lex!(lex_number, "123>", ok![number!("123"), loc!(4, 1)]);
        assert_lex!(lex_number, "123<", ok![number!("123"), loc!(4, 1)]);

        assert_lex!(
            "<&123",
            ok![
                vec![Token::read_copy(loc!(1, 1)), number!("123", loc!(3, 1))],
                loc!(6, 1)
            ]
        );

        assert_lex!(
            ">&123",
            ok![
                vec![Token::write_copy(loc!(1, 1)), number!("123", loc!(3, 1))],
                loc!(6, 1)
            ]
        );
    }

    #[test]
    fn test_lex_redirect() {
        assert_lex!(
            lex_redirect,
            "<",
            ok![Token::read_from(loc!(1, 1)), loc!(2, 1)]
        );
        assert_lex!(
            lex_redirect,
            "<<",
            ok![Token::here_document(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect,
            "<<<",
            ok![Token::here_string(loc!(1, 1)), loc!(4, 1)]
        );
        assert_lex!(
            lex_redirect,
            "<>",
            ok![Token::read_write(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect,
            "<&-",
            ok![Token::read_close(loc!(1, 1)), loc!(4, 1)]
        );
        assert_lex!(
            lex_redirect,
            "<&",
            ok![Token::read_copy(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect,
            ">",
            ok![Token::write_to(loc!(1, 1)), loc!(2, 1)]
        );
        assert_lex!(
            lex_redirect,
            ">>",
            ok![Token::append(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect,
            ">|",
            ok![Token::force_write_to(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect,
            ">&-",
            ok![Token::write_close(loc!(1, 1)), loc!(4, 1)]
        );
        assert_lex!(
            lex_redirect,
            ">&",
            ok![Token::write_both(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect,
            ">&1",
            ok![Token::write_copy(loc!(1, 1)), loc!(3, 1)]
        );
    }

    #[test]
    fn test_lex_redirect_less() {
        assert_lex!(
            lex_redirect_less,
            "<",
            ok![Token::read_from(loc!(1, 1)), loc!(2, 1)]
        );
        assert_lex!(
            lex_redirect_less,
            "<<",
            ok![Token::here_document(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect_less,
            "<<<",
            ok![Token::here_string(loc!(1, 1)), loc!(4, 1)]
        );
        assert_lex!(
            lex_redirect_less,
            "<>",
            ok![Token::read_write(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect_less,
            "<&-",
            ok![Token::read_close(loc!(1, 1)), loc!(4, 1)]
        );
        assert_lex!(
            lex_redirect_less,
            "<&",
            ok![Token::read_copy(loc!(1, 1)), loc!(3, 1)]
        );
    }

    #[test]
    fn test_redirect_grater() {
        assert_lex!(
            lex_redirect_grater,
            ">",
            ok![Token::write_to(loc!(1, 1)), loc!(2, 1)]
        );
        assert_lex!(
            lex_redirect_grater,
            ">>",
            ok![Token::append(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect_grater,
            ">|",
            ok![Token::force_write_to(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect_grater,
            ">&-",
            ok![Token::write_close(loc!(1, 1)), loc!(4, 1)]
        );
        assert_lex!(
            lex_redirect_grater,
            ">&",
            ok![Token::write_both(loc!(1, 1)), loc!(3, 1)]
        );
        assert_lex!(
            lex_redirect_grater,
            ">&1",
            ok![Token::write_copy(loc!(1, 1)), loc!(3, 1)]
        );
    }

    #[test]
    fn test_lex_doller() {
        assert_lex!(lex_dollar, "$foo$bar", ok![var!("foo"), loc!(5, 1)]);
        assert_lex!(lex_dollar, "$foo_bar", ok![var!("foo_bar"), loc!(9, 1)]);
        assert_lex!(lex_dollar, "$foo-bar", ok![var!("foo"), loc!(5, 1)]);
        assert_lex!(lex_dollar, "$", ok![normal_word!("$"), loc!(2, 1)]);
    }

    #[test]
    fn test_lex_backquote() {
        assert_lex!(
            lex_backquote,
            "`foo\nbar\\`baz`",
            ok![cmd!("foo\nbar`baz"), loc!(10, 2)]
        );
        assert_lex!(
            lex_backquote,
            "`foobar\\ baz`",
            ok![cmd!("foobar\\ baz"), loc!(14, 1)]
        );
        assert_lex!(
            lex_backquote,
            "`foo$(bar\\`baz\\`)`",
            ok![cmd!("foo$(bar`baz`)"), loc!(19, 1)]
        );
        assert_lex!(lex_backquote, "`foobar", Err(Error::eof(loc!(8, 1))));
        assert_lex!(lex_backquote, "`", Err(Error::eof(loc!(2, 1))));
    }

    #[test]
    fn test_lex_single_quote() {
        assert_lex!(
            lex_single_quote,
            "'foo\nbar\\'baz'",
            ok![literal_word!("foo\nbar'baz"), loc!(10, 2)]
        );
        assert_lex!(
            lex_single_quote,
            "'foo`bar`baz'",
            ok![literal_word!("foo`bar`baz"), loc!(14, 1)]
        );
        assert_lex!(
            lex_single_quote,
            r#"'foo"bar"baz'"#,
            ok![literal_word!(r#"foo"bar"baz"#), loc!(14, 1)]
        );
        assert_lex!(
            lex_single_quote,
            "'foobar",
            Err(Error::eof(loc!(8, 1)))
        );
    }

    #[test]
    fn test_lex_doubule_quote() {
        assert_lex!(
            lex_double_quote,
            "\"foo\nbar\\\"baz\"",
            ok![vec![quote_word!("foo\nbar\"baz")], loc!(10, 2)]
        );
        assert_lex!(
            lex_double_quote,
            r#""foo'bar'baz""#,
            ok![vec![quote_word!("foo'bar'baz")], loc!(14, 1)]
        );
        assert_lex!(
            lex_double_quote,
            r#""foo`bar`baz""#,
            ok![
                vec![
                    quote_word!("foo"),
                    cmd!("bar", loc!(5, 1)),
                    quote_word!("baz", loc!(10, 1))
                ],
                loc!(14, 1)
            ]
        );
        assert_lex!(
            lex_double_quote,
            r#""foo${bar}baz""#,
            ok![
                vec![
                    quote_word!("foo"),
                    param!("bar", loc!(5, 1)),
                    quote_word!("baz", loc!(11, 1))
                ],
                loc!(15, 1)
            ]
        );
    }

    #[test]
    fn test_lex_word() {
        assert_lex!(
            lex_word,
            "foo\\bar\\baz",
            ok![normal_word!("foo\\bar\\baz"), loc!(12, 1)]
        );
        assert_lex!(
            lex_word,
            "foo\\\nbar\\ baz",
            ok![normal_word!("foo\nbar baz"), loc!(9, 2)]
        );
        assert_lex!(
            lex_word,
            "foo`bar`baz",
            ok![normal_word!("foo"), loc!(4, 1)]
        );
        assert_lex!(
            lex_word,
            "foo${bar}baz",
            ok![normal_word!("foo"), loc!(4, 1)]
        );
    }

    #[test]
    fn test_lex_parameter() {
        assert_lex!(
            lex_parameter,
            "${foobar}",
            ok![param!("foobar"), loc![10, 1]]
        );
        assert_lex!(lex_parameter, "${foobar", Err(Error::eof(loc![9, 1])));
    }

    #[test]
    fn test_lex_command_substitute() {
        assert_lex!(
            lex_command_substitute,
            "$(foo\nbar)",
            ok![cmd!("foo\nbar"), loc!(5, 2)]
        );
        assert_lex!(
            lex_command_substitute,
            "$(foo$(bar$(baz)))",
            ok![cmd!("foo$(bar$(baz))"), loc!(19, 1)]
        );
        assert_lex!(
            lex_command_substitute,
            "$(foo`bar\\`baz`)",
            ok![cmd!("foo`bar\\`baz`"), loc!(17, 1)]
        );
        assert_lex!(
            lex_command_substitute,
            "$(foobar",
            Err(Error::eof(loc!(9, 1)))
        );
        assert_lex!(
            lex_command_substitute,
            "$(foobar$(bar)",
            Err(Error::eof(loc!(15, 1)))
        );
    }

    #[test]
    fn test_lex_variable() {
        assert_lex!(lex_variable, "$foo$bar", ok![var!("foo"), loc!(5, 1)]);
        assert_lex!(lex_variable, "$foo_bar", ok![var!("foo_bar"), loc!(9, 1)]);
        assert_lex!(lex_variable, "$foo-bar", ok![var!("foo"), loc!(5, 1)]);
        assert_lex!(lex_variable, "$$foobar", ok![var!("$"), loc!(3, 1)]);
    }

    #[test]
    fn test_lex_comment() {
        assert_lex!(
            lex_comment,
            "#foo bar",
            ok![
                Token::comment("foo bar".to_string(), loc!(1, 1)),
                loc!(9, 1)
            ]
        );

        assert_lex!(
            "foo # bar",
            ok![
                vec![
                    normal_word!("foo", loc!(1, 1)),
                    Token::space(loc!(4, 1)),
                    Token::comment(" bar".to_string(), loc!(5, 1))
                ],
                loc!(10, 1)
            ]
        );
    }
}
