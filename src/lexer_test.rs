#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::ParseError;
    use crate::*;

    macro_rules! lex {
        ($f: ident) => {
            |s| {
                let mut lexer = Lexer::new(s);
                lexer.$f().map(|t| (t, lexer.location()))
            }
        };
        ($($b: ident)+, $f: ident) => {
            |s| {
                let mut lexer = Lexer::new(s);
                $(let _ = lexer.$b();)+
                lexer.$f().map(|t| (t, lexer.location()))
            }
        };
        ($f: ident, $s: expr, $t: expr) => {
            |s| {
                let s = format!("{}{}",$s, s);
                let mut lexer = Lexer::new(&*s);
                lexer.push($t($s.to_string(), loc!(1, 1)));
                lexer.pos += $s.len();
                lexer.column += lexer.pos;
                lexer.$f().map(|t| (t, lexer.location()))
            }
        };
    }

    #[test]
    fn test_lex_keyword() {
        test_case! {
            lex => {
                "if"     => Ok(vec![Token::if_keyword(loc!(1, 1))]),
                "then"   => Ok(vec![Token::then_keyword(loc!(1, 1))]),
                "fi"     => Ok(vec![Token::fi_keyword(loc!(1, 1))]),
                "elif"   => Ok(vec![Token::elif_keyword(loc!(1, 1))]),
                "elsif"  => Ok(vec![Token::elsif_keyword(loc!(1, 1))]),
                "end"    => Ok(vec![Token::end_keyword(loc!(1, 1))]),
                "unless" => Ok(vec![Token::unless_keyword(loc!(1, 1))]),
                "while"  => Ok(vec![Token::while_keyword(loc!(1, 1))]),
                "do"     => Ok(vec![Token::do_keyword(loc!(1, 1))]),
                "done"   => Ok(vec![Token::done_keyword(loc!(1, 1))]),
                "echo if" => Ok(vec![
                    normal_word!("echo"),
                    Token::space(loc!(5, 1)),
                    normal_word!("if", loc!(6, 1)),
                ]),
            },
        }
    }

    #[test]
    fn test_lex_space() {
        test_case! {
            lex!(lex_space) => {
                "  \t  " => Ok((Token::space(loc!(1,1)), loc!(6, 1))),
            },
        }
    }

    #[test]
    fn test_lex_newline() {
        test_case! {
            lex!(lex_newline) => {
                "\n\n\n \t" => Ok((Token::newline(loc!(1,1)), loc!(1, 4))),
            },
        }
    }

    #[test]
    fn test_lex_semicolon() {
        test_case! {
            lex!(lex_semicolon) => {
                ";;;\n\t" => Ok((Token::termination(loc!(1,1)), loc!(4, 1))),
            },
        }
    }

    #[test]
    fn test_lex_ampersand() {
        test_case! {
            lex!(lex_ampersand) => {
                "&"   => Ok((Token::background(loc!(1, 1)),  loc!(2, 1))),
                "&&"  => Ok((Token::and(loc!(1, 1)),         loc!(3, 1))),
                "&>"  => Ok((Token::write_both(loc!(1, 1)),  loc!(3, 1))),
                "&>>" => Ok((Token::append_both(loc!(1, 1)), loc!(4, 1))),
            },
        }
    }

    #[test]
    fn test_lex_hyphen() {
        test_case! {
            lex!(lex_hyphen) => {
                "-" => Ok((normal_word!("-"), loc!(2, 1))),
            },
            lex!(lex_hyphen, "123", Token::number) => {
                "-" => Ok((Token::hyphen(loc!(4, 1)), loc!(5, 1))),
            },
        };
    }

    #[test]
    fn test_lex_vertical_line() {
        test_case! {
            lex!(lex_vertical_line) => {
                "|"  => Ok((Token::pipe(loc!(1, 1)),      loc!(2, 1))),
                "||" => Ok((Token::or(loc!(1, 1)),        loc!(3, 1))),
                "|&" => Ok((Token::pipe_both(loc!(1, 1)), loc!(3, 1))),
            },
        };
    }

    #[test]
    fn test_lex_number() {
        test_case! {
            lex!(lex_number) => {
                "123"  => Ok((normal_word!("123"), loc!(4, 1))),
                "123>" => Ok((number!("123"), loc!(4, 1))),
                "123<" => Ok((number!("123"), loc!(4, 1))),
            },
            lex!(lex_number, "<&", |_, l| Token::read_copy(l)) => {
                "123" => Ok((number!("123", loc!(3, 1)), loc!(6, 1))),
            },
            lex!(lex_number, ">&", |_, l| Token::write_copy(l)) => {
                "123" => Ok((number!("123", loc!(3, 1)), loc!(6, 1))),
            },
            lex!(lex_number, ">", |_, l| Token::write_to(l)) => {
                "123" => Ok((normal_word!("123", loc!(2, 1)), loc!(5, 1))),
            },
        };
    }

    #[test]
    fn test_lex_redirect() {
        test_case! {
            lex!(lex_redirect) => {
                "<"   => Ok((Token::read_from(loc!(1, 1)), loc!(2, 1)))
                "<<"  => Ok((Token::here_document(loc!(1, 1)), loc!(3, 1)))
                "<<<" => Ok((Token::here_string(loc!(1, 1)), loc!(4, 1)))
                "<>"  => Ok((Token::read_write(loc!(1, 1)), loc!(3, 1))),
                "<&-" => Ok((Token::read_close(loc!(1, 1)), loc!(4, 1))),
                "<&"  => Ok((Token::read_copy(loc!(1, 1)), loc!(3, 1))),
                ">"   => Ok((Token::write_to(loc!(1, 1)), loc!(2, 1)))
                ">>"  => Ok((Token::append(loc!(1, 1)), loc!(3, 1)))
                ">|"  => Ok((Token::force_write_to(loc!(1, 1)), loc!(3, 1)))
                ">&-" => Ok((Token::write_close(loc!(1, 1)), loc!(4, 1))),
                ">&"  => Ok((Token::write_both(loc!(1, 1)), loc!(3, 1))),
                ">&1" => Ok((Token::write_copy(loc!(1, 1)), loc!(3, 1))),
            },
        }
    }

    #[test]
    fn test_lex_redirect_less() {
        test_case! {
            lex!(lex_redirect_less) => {
                "<"   => Ok((Token::read_from(loc!(1, 1)), loc!(2, 1)))
                "<<"  => Ok((Token::here_document(loc!(1, 1)), loc!(3, 1)))
                "<<<" => Ok((Token::here_string(loc!(1, 1)), loc!(4, 1)))
                "<>"  => Ok((Token::read_write(loc!(1, 1)), loc!(3, 1))),
                "<&-" => Ok((Token::read_close(loc!(1, 1)), loc!(4, 1))),
                "<&"  => Ok((Token::read_copy(loc!(1, 1)), loc!(3, 1))),
            },
        }
    }

    #[test]
    fn test_redirect_grater() {
        test_case! {
            lex!(lex_redirect_grater) => {
                ">"   => Ok((Token::write_to(loc!(1, 1)), loc!(2, 1)))
                ">>"  => Ok((Token::append(loc!(1, 1)), loc!(3, 1)))
                ">|"  => Ok((Token::force_write_to(loc!(1, 1)), loc!(3, 1)))
                ">&-" => Ok((Token::write_close(loc!(1, 1)), loc!(4, 1))),
                ">&"  => Ok((Token::write_both(loc!(1, 1)), loc!(3, 1))),
                ">&1" => Ok((Token::write_copy(loc!(1, 1)), loc!(3, 1))),
            },
        }
    }

    #[test]
    fn test_lex_doller() {
        test_case! {
            lex!(lex_dollar) => {
                "$foo$bar" => Ok((var!("foo"), loc!(5,1))),
                "$foo_bar" => Ok((var!("foo_bar"), loc!(9,1))),
                "$foo-bar" => Ok((var!("foo"), loc!(5,1))),
                "$" =>        Ok((normal_word!("$"), loc!(2,1))),
            },
        }
    }

    #[test]
    fn test_lex_backquote() {
        test_case! {
            lex!(lex_backquote) => {
                "`foo\nbar\\`baz`"     => Ok((cmd!("foo\nbar`baz"),   loc!(10, 2))),
                "`foobar\\ baz`"       => Ok((cmd!("foobar\\ baz"),   loc!(14, 1))),
                "`foo$(bar\\`baz\\`)`" => Ok((cmd!("foo$(bar`baz`)"), loc!(19, 1))),
                "`foobar"              => Err(ParseError::eof(loc!(8, 1))),
                "`"                    => Err(ParseError::eof(loc!(2, 1))),
            },
        }
    }

    #[test]
    fn test_lex_single_quote() {
        test_case! {
            lex!(lex_single_quote) => {
                "'foo\nbar\\'baz'" => Ok((literal_word!("foo\nbar'baz"),  loc!(10, 2))),
                "'foo`bar`baz'"    => Ok((literal_word!("foo`bar`baz"),loc!(14, 1))),
                r#"'foo"bar"baz'"# => Ok((literal_word!(r#"foo"bar"baz"#),loc!(14, 1))),
                "'foobar"          => Err(ParseError::eof(loc!(8, 1))),
            },
        }
    }

    #[test]
    fn test_lex_doubule_quote() {
        test_case! {
            lex!(lex_double_quote) => {
                "\"foo\nbar\\\"baz\"" => Ok((vec![quote_word!("foo\nbar\"baz")], loc!(10, 2))),
                r#""foo'bar'baz""#    => Ok((vec![quote_word!("foo'bar'baz")],   loc!(14, 1))),
                r#""foo`bar`baz""#    => Ok((vec![
                    quote_word!("foo"), cmd!("bar", loc!(5, 1)), quote_word!("baz", loc!(10, 1))
                    ], loc!(14, 1)
                )),
                r#""foo${bar}baz""#    => Ok((vec![
                    quote_word!("foo"), param!("bar", loc!(5, 1)), quote_word!("baz", loc!(11, 1))
                    ], loc!(15, 1)
                )),
            },
        }
    }

    #[test]
    fn test_lex_word() {
        test_case! {
            lex!(lex_word) => {
                "foo\\bar\\baz"    => Ok((normal_word!("foo\\bar\\baz"), loc!(12, 1))),
                "foo\\\nbar\\ baz" => Ok((normal_word!("foo\nbar baz"), loc!(9, 2))),
                "foo`bar`baz"      => Ok((normal_word!("foo"), loc!(4, 1))),
                "foo${bar}baz"     => Ok((normal_word!("foo"), loc!(4, 1))),
            },
        }
    }

    #[test]
    fn test_lex_parameter() {
        test_case! {
            lex!(lex_parameter) => {
                "${foobar}" => Ok((param!("foobar"), loc![10, 1])),
                "${foobar"  => Err(ParseError::eof(loc![9, 1])),
            },
        }
    }

    #[test]
    fn test_lex_command_substitute() {
        test_case! {
            lex!(lex_command_substitute) => {
                "$(foo\nbar)"        => Ok((cmd!("foo\nbar"),        loc!(5, 2))),
                "$(foo$(bar$(baz)))" => Ok((cmd!("foo$(bar$(baz))"), loc!(19, 1))),
                "$(foo`bar\\`baz`)"  => Ok((cmd!("foo`bar\\`baz`"),  loc!(17, 1))),
                "$(foobar"           => Err(ParseError::eof(loc!(9,1))),
                "$(foobar$(bar)"     => Err(ParseError::eof(loc!(15,1))),
            },
        };
    }

    #[test]
    fn test_lex_variable() {
        test_case! {
            lex!(lex_variable) => {
                "$foo$bar" => Ok((var!("foo"),     loc!(5,1))),
                "$foo_bar" => Ok((var!("foo_bar"), loc!(9,1))),
                "$foo-bar" => Ok((var!("foo"),     loc!(5,1))),
            },
        }
    }
}
