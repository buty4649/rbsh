#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::redirect::Redirect;
    use crate::parser::word::{WordKind, WordList};
    use crate::parser::Location;
    use crate::*;

    macro_rules! got {
        ($e: expr) => {
            |t: Vec<Token>| $e(&mut t.into_iter().peekable())
        };
    }

    macro_rules! got_all {
        ($e: expr) => {
            |t: Vec<Token>| {
                let mut result = vec![];
                let mut t = t.into_iter().peekable();
                loop {
                    match $e(&mut t) {
                        Ok(None) => break result,
                        Err(c) => {
                            result.push(Err(c));
                            break result;
                        }
                        c => result.push(c),
                    }
                }
            }
        };
    }

    macro_rules! ok {
        ($e: expr) => {
            Ok(Some($e))
        };
    }

    macro_rules! simple_command {
        ($c: expr, $r: expr, $b: expr) => {
            UnitKind::SimpleCommand {
                command: $c,
                redirect: $r,
                background: $b,
            }
        };
    }

    macro_rules! connecter_pipe {
        ($left: expr, $right: expr, $background: expr) => {
            UnitKind::Connecter {
                left: Box::new($left),
                right: Box::new($right),
                kind: ConnecterKind::Pipe,
                background: $background,
            }
        };
    }

    macro_rules! connecter_pipe_both {
        ($left: expr, $right: expr, $background: expr) => {
            UnitKind::Connecter {
                left: Box::new($left),
                right: Box::new($right),
                kind: ConnecterKind::PipeBoth,
                background: $background,
            }
        };
    }

    macro_rules! connecter_and {
        ($left: expr, $right: expr, $background: expr) => {
            UnitKind::Connecter {
                left: Box::new($left),
                right: Box::new($right),
                kind: ConnecterKind::And,
                background: $background,
            }
        };
    }

    macro_rules! connecter_or {
        ($left: expr, $right: expr, $background: expr) => {
            UnitKind::Connecter {
                left: Box::new($left),
                right: Box::new($right),
                kind: ConnecterKind::Or,
                background: $background,
            }
        };
    }

    macro_rules! w {
        ($($e: expr$(,)?)+) => {{
            WordList::from(vec![$($e,)+])
        }};
    }

    #[test]
    fn test_parse_command() {
        test_case! {
            got_all!(parse_command) => {
                // foo &
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::background(loc!(5, 1)),
                ] => vec![
                    ok![simple_command!(vec![w!["foo"]], vec![], true)],
                ],
                // foo & bar
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::background(loc!(5, 1)),
                    Token::space(loc!(6, 1)),
                    normal_word!("bar", loc!(7, 1)),
                ] => vec![
                    ok![simple_command!(vec![w!["foo"]], vec![], true)],
                    ok![simple_command!(vec![w![normal_word!("bar", loc!(7, 1))]], vec![], false)],
                ],
                // foo ;
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::termination(loc!(5, 1)),
                ] => vec![
                    ok![simple_command!(vec![w!["foo"]], vec![], false)],
                ],
                // foo ; bar
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::termination(loc!(5, 1)),
                    Token::space(loc!(6, 1)),
                    normal_word!("bar", loc!(7, 1)),
                ] => vec![
                    ok![simple_command!(vec![w!["foo"]], vec![], false)],
                    ok![simple_command!(vec![w![normal_word!("bar", loc!(7, 1))]], vec![], false)],
                ],
                // foo | bar
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::pipe(loc!(5, 1)),
                    Token::space(loc!(6, 1)),
                    normal_word!("bar", loc!(7, 1)),
                ] => vec![ok![
                    connecter_pipe![
                        simple_command!(vec![w!["foo"]], vec![], false),
                        simple_command!(vec![w![normal_word!("bar", loc!(7, 1))]], vec![], false),
                        false
                    ]
                ]],
                // foo |& bar
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::pipe_both(loc!(5, 1)),
                    Token::space(loc!(7, 1)),
                    normal_word!("bar", loc!(8, 1)),
                ] => vec![ok![
                    connecter_pipe_both![
                        simple_command!(vec![w!["foo"]], vec![], false),
                        simple_command!(vec![w![normal_word!("bar", loc!(8, 1))]], vec![], false),
                        false
                    ]
                ]],
                // foo && bar
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::and(loc!(5, 1)),
                    Token::space(loc!(7, 1)),
                    normal_word!("bar", loc!(8, 1)),
                ] => vec![ok![
                    connecter_and![
                        simple_command!(vec![w!["foo"]], vec![], false),
                        simple_command!(vec![w![normal_word!("bar", loc!(8, 1))]], vec![], false),
                        false
                    ]
                ]],
                // foo || bar
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::or(loc!(5, 1)),
                    Token::space(loc!(7, 1)),
                    normal_word!("bar", loc!(8, 1)),
                ] => vec![ok![
                    connecter_or![
                        simple_command!(vec![w!["foo"]], vec![], false),
                        simple_command!(vec![w![normal_word!("bar", loc!(8, 1))]], vec![], false),
                        false
                    ]
                ]],
                // foo && bar || baz &
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::and(loc!(5, 1)),
                    Token::space(loc!(7, 1)),
                    normal_word!("bar", loc!(8, 1)),
                    Token::space(loc!(11, 1)),
                    Token::or(loc!(12, 1)),
                    Token::space(loc!(13, 1)),
                    normal_word!("baz", loc!(14, 1)),
                    Token::space(loc!(17, 1)),
                    Token::background(loc!(18, 1)),
                ] => vec![ok![
                    connecter_and![
                        simple_command!(vec![w!["foo"]], vec![], false),
                        connecter_or![
                            simple_command!(vec![w![normal_word!("bar", loc!(8, 1))]], vec![], false),
                            simple_command!(vec![w![normal_word!("baz", loc!(14, 1))]], vec![], false),
                            false
                        ],
                        true
                    ]
                ]],


                // & foo
                vec![
                    Token::background(loc!(1, 1)),
                    Token::space(loc!(2, 1)),
                    normal_word!("foo", loc!(3, 1)),
                ] => vec![
                    Err(ParseError::unexpected_token(Token::background(loc!(1, 1))))
                ],

                // &&
                vec![
                    Token::and(loc!(1, 1)),
                ] => vec![
                    Err(ParseError::unexpected_token(Token::and(loc!(1, 1))))
                ],

                // foo &&
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::and(loc!(5, 1)),
                ] => vec![
                    Err(ParseError::unexpected_token(Token::and(loc!(5, 1))))
                ],

                // foo && &
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::and(loc!(5, 1)),
                    Token::space(loc!(6, 1)),
                    Token::background(loc!(7, 1)),
                ] => vec![
                    Err(ParseError::unexpected_token(Token::background(loc!(7, 1))))
                ],
            },
        }
    }

    #[test]
    fn test_parse_shell_command() {
        test_case! {
            got!(parse_shell_command) => {
                // foo > bar 2>&1
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::write_to(loc!(5, 1)),
                    Token::space(loc!(6, 1)),
                    normal_word!("bar", loc!(7, 1)),
                    Token::space(loc!(10, 1)),
                    number!("2", loc!(11, 1)),
                    Token::write_copy(loc!(12, 1)),
                    number!("1", loc!(15, 1)),
                ] => ok![simple_command!(
                    vec![w!["foo"]],
                    vec![
                        Redirect::write_to(1, w![normal_word!("bar", loc!(7, 1))], false, loc!(5, 1)),
                        Redirect::write_copy(1, 2, false, loc!(11, 1)),
                    ],
                    false
                )],
            },
        }
    }

    #[test]
    fn test_parse_simple_command() {
        test_case! {
            got!(parse_simple_command) => {
                // foobar
                vec![normal_word!("foo"), normal_word!("bar", loc!(4, 1))] =>
                    ok![simple_command!(vec![w!["foo", "bar"]], vec![], false)],
                // foo bar
                vec![normal_word!("foo"), Token::space(loc!(4,1)), normal_word!("bar", loc!(5, 1))] =>
                    ok![simple_command!(vec![
                        w!["foo"],
                        w![normal_word!("bar", loc!(5,1))]], vec![], false
                    )],
                // foo > bar 2>&1
                vec![
                    normal_word!("foo"),
                    Token::space(loc!(4, 1)),
                    Token::write_to(loc!(5, 1)),
                    Token::space(loc!(6, 1)),
                    normal_word!("bar", loc!(7, 1)),
                    Token::space(loc!(10, 1)),
                    number!("2", loc!(11, 1)),
                    Token::write_copy(loc!(12, 1)),
                    number!("1", loc!(15, 1)),
                ] => ok![simple_command!(
                    vec![w!["foo"]],
                    vec![
                        Redirect::write_to(1, w![normal_word!("bar", loc!(7, 1))], false, loc!(5, 1)),
                        Redirect::write_copy(1, 2, false, loc!(11, 1)),
                    ],
                    false
                )],
                // > bar foo
                vec![
                    Token::write_to(loc!(1, 1)),
                    Token::space(loc!(2, 1)),
                    normal_word!("bar", loc!(3, 1)),
                    Token::space(loc!(6, 1)),
                    normal_word!("foo", loc!(7, 1)),
                ] => ok![simple_command!(
                    vec![w![normal_word!("foo", loc!(7, 1))]],
                    vec![Redirect::write_to(1, w![normal_word!("bar", loc!(3, 1))], false, loc!(1, 1))],
                    false
                )],
            },
        }
    }
}
