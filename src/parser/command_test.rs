#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        literal_word, loc,
        location::Location,
        normal_word,
        parser::{
            lexer::lex,
            redirect::Redirect,
            token::Token,
            word::{WordKind, WordList},
        },
        quote_word,
    };

    macro_rules! lex {
        ($e: expr) => {
            lex($e).unwrap()
        };
    }

    macro_rules! assert_parse {
        ($f: ident, $e: expr, $expect: expr) => {
            let mut t = TokenReader::new(lex!($e));
            let got = $f(&mut t);
            assert_eq!($expect, got);
        };
    }

    macro_rules! assert_parse_all {
        ($f: ident, $e: expr, $expect: expr) => {
            let t = TokenReader::new(lex!($e));
            let got = |mut t| {
                let mut got = vec![];
                loop {
                    match $f(&mut t) {
                        Ok(None) => return Ok(Some(got)),
                        Ok(Some(c)) => got.push(c),
                        Err(e) => return Err(e),
                    }
                }
            };
            assert_eq!($expect, got(t));
        };
    }

    macro_rules! ok {
        ($e: expr) => {
            Ok(Some($e))
        };
    }

    macro_rules! unit {
        ($e: expr, $b: expr) => {
            Unit::new($e, $b)
        };
    }

    macro_rules! simple_command {
        ($c: expr, $r: expr) => {
            UnitKind::SimpleCommand {
                command: $c,
                redirect: $r,
            }
        };

        ($c: expr) => {
            simple_command!($c, vec![])
        };
    }

    macro_rules! if_stmt {
        ($c: expr, $t: expr, $f: expr, $r: expr) => {
            UnitKind::If {
                condition: Box::new($c),
                true_case: $t,
                false_case: $f,
                redirect: $r,
            }
        };

        ($c: expr, $t: expr) => {
            if_stmt!($c, $t, None, vec![])
        };

        ($c: expr, $t: expr, $f: expr) => {
            if_stmt!($c, $t, Some($f), vec![])
        };
    }

    macro_rules! unless_stmt {
        ($c: expr, $f: expr, $t: expr, $r: expr) => {
            UnitKind::Unless {
                condition: Box::new($c),
                false_case: $f,
                true_case: $t,
                redirect: $r,
            }
        };

        ($c: expr, $f: expr) => {
            unless_stmt!($c, $f, None, vec![])
        };

        ($c: expr, $f: expr, $t: expr) => {
            unless_stmt!($c, $f, Some($t), vec![])
        };
    }

    macro_rules! while_stmt {
        ($c: expr, $a: expr, $r: expr) => {
            UnitKind::While {
                condition: Box::new($c),
                command: $a,
                redirect: $r,
            }
        };

        ($c: expr, $a: expr) => {
            while_stmt!($c, $a, vec![])
        };
    }

    macro_rules! until_stmt {
        ($c: expr, $a: expr, $r: expr) => {
            UnitKind::Until {
                condition: Box::new($c),
                command: $a,
                redirect: $r,
            }
        };

        ($c: expr, $a: expr) => {
            until_stmt!($c, $a, vec![])
        };
    }

    macro_rules! for_stmt {
        ($i: expr, $l: expr, $c: expr, $r: expr) => {
            UnitKind::For {
                identifier: $i,
                list: $l,
                command: $c,
                redirect: $r,
            }
        };

        ($i: expr, $c: expr) => {
            for_stmt!($i, None, $c, vec![])
        };

        ($i: expr, $l: expr, $c: expr) => {
            for_stmt!($i, Some($l), $c, vec![])
        };
    }

    macro_rules! connecter_pipe {
        ($left: expr, $right: expr) => {
            UnitKind::Connecter {
                left: Box::new($left),
                right: Box::new($right),
                kind: ConnecterKind::Pipe,
            }
        };
    }

    macro_rules! connecter_pipe_both {
        ($left: expr, $right: expr) => {
            UnitKind::Connecter {
                left: Box::new($left),
                right: Box::new($right),
                kind: ConnecterKind::PipeBoth,
            }
        };
    }

    macro_rules! connecter_and {
        ($left: expr, $right: expr) => {
            UnitKind::Connecter {
                left: Box::new($left),
                right: Box::new($right),
                kind: ConnecterKind::And,
            }
        };
    }

    macro_rules! connecter_or {
        ($left: expr, $right: expr) => {
            UnitKind::Connecter {
                left: Box::new($left),
                right: Box::new($right),
                kind: ConnecterKind::Or,
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
        assert_parse!(
            parse_command,
            "foo &",
            ok![unit![simple_command!(vec![w!["foo"]], vec![]), true]]
        );

        assert_parse!(
            parse_command,
            "foo ;",
            ok![unit!(simple_command!(vec![w!["foo"]], vec![]), false)]
        );

        assert_parse!(
            parse_command,
            "foo | bar",
            ok![unit![
                connecter_pipe!(
                    unit![simple_command!(vec![w!["foo"]], vec![]), false],
                    unit![
                        simple_command!(vec![w![normal_word!("bar", loc!(7, 1))]], vec![]),
                        false
                    ]
                ),
                false
            ]]
        );

        assert_parse!(
            parse_command,
            "foo |& bar",
            ok![unit![
                connecter_pipe_both!(
                    unit![simple_command!(vec![w!["foo"]], vec![]), false],
                    unit![
                        simple_command!(vec![w![normal_word!("bar", loc!(8, 1))]], vec![]),
                        false
                    ]
                ),
                false
            ]]
        );

        assert_parse!(
            parse_command,
            "foo && bar",
            ok![unit![
                connecter_and!(
                    unit![simple_command!(vec![w!["foo"]], vec![]), false],
                    unit![
                        simple_command!(vec![w![normal_word!("bar", loc!(8, 1))]], vec![]),
                        false
                    ]
                ),
                false
            ]]
        );

        assert_parse!(
            parse_command,
            "foo || bar",
            ok![unit![
                connecter_or!(
                    unit![simple_command!(vec![w!["foo"]], vec![]), false],
                    unit![
                        simple_command!(vec![w![normal_word!("bar", loc!(8, 1))]], vec![]),
                        false
                    ]
                ),
                false
            ]]
        );

        assert_parse!(
            parse_command,
            "& foo",
            Err(ShellError::unexpected_token(Token::background(loc!(1, 1))))
        );

        assert_parse!(
            parse_command,
            "&&",
            Err(ShellError::unexpected_token(Token::and(loc!(1, 1))))
        );

        assert_parse!(
            parse_command,
            "foo &&",
            Err(ShellError::unexpected_token(Token::and(loc!(5, 1))))
        );

        assert_parse!(
            parse_command,
            "foo && &",
            Err(ShellError::unexpected_token(Token::background(loc!(8, 1))))
        );

        assert_parse!(
            parse_command,
            "if foo; bar; end > baz 2>&1 &",
            ok![unit![
                if_stmt!(
                    unit![
                        simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                        false
                    ],
                    vec!(unit![
                        simple_command!(vec![w![normal_word!("bar", loc!(9, 1))]]),
                        false
                    ]),
                    None,
                    vec![
                        Redirect::write_to(
                            1,
                            w![normal_word!("baz", loc!(20, 1))],
                            false,
                            loc!(18, 1)
                        ),
                        Redirect::copy(1, 2, false, loc!(24, 1)),
                    ]
                ),
                true
            ]]
        );

        assert_parse!(
            parse_command,
            "if foo; bar; end && if baz; foo; end",
            ok![unit![
                connecter_and!(
                    unit![
                        if_stmt![
                            unit![
                                simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                                false
                            ],
                            vec!(unit![
                                simple_command!(vec![w![normal_word!("bar", loc!(9, 1))]]),
                                false
                            ])
                        ],
                        false
                    ],
                    unit![
                        if_stmt![
                            unit![
                                simple_command!(vec![w![normal_word!("baz", loc!(24, 1))]]),
                                false
                            ],
                            vec!(unit![
                                simple_command!(vec![w![normal_word!("foo", loc!(29, 1))]]),
                                false
                            ])
                        ],
                        false
                    ]
                ),
                false
            ]]
        );

        assert_parse_all!(
            parse_command,
            "foo & bar",
            ok![vec![
                unit![simple_command!(vec![w!["foo"]], vec![]), true],
                unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(7, 1))]], vec![]),
                    false
                ],
            ]]
        );

        assert_parse_all!(
            parse_command,
            "foo ; bar",
            ok![vec![
                unit![simple_command!(vec![w!["foo"]], vec![]), false],
                unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(7, 1))]], vec![]),
                    false
                ],
            ]]
        );

        assert_parse_all!(
            parse_command,
            "foo && bar || baz &",
            ok![vec![unit![
                connecter_and![
                    unit![simple_command!(vec![w!["foo"]], vec![]), false],
                    unit![
                        connecter_or![
                            unit![
                                simple_command!(vec![w![normal_word!("bar", loc!(8, 1))]], vec![]),
                                false
                            ],
                            unit![
                                simple_command!(vec![w![normal_word!("baz", loc!(15, 1))]], vec![]),
                                false
                            ]
                        ],
                        false
                    ]
                ],
                true
            ]]]
        );
    }

    #[test]
    fn test_parse_shell_command() {
        assert_parse!(
            parse_shell_command,
            "if foo; bar; end > baz 2>&1",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(9, 1))]]),
                    false
                ]),
                None,
                vec![
                    Redirect::write_to(1, w![normal_word!("baz", loc!(20, 1))], false, loc!(18, 1)),
                    Redirect::copy(1, 2, false, loc!(24, 1)),
                ]
            ]]
        );

        assert_parse!(
            parse_shell_command,
            "unless foo; bar; end > baz 2>&1",
            ok![unless_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(8, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(13, 1))]]),
                    false
                ]),
                None,
                vec![
                    Redirect::write_to(1, w![normal_word!("baz", loc!(24, 1))], false, loc!(22, 1)),
                    Redirect::copy(1, 2, false, loc!(28, 1)),
                ]
            ]]
        );

        assert_parse!(
            parse_shell_command,
            "while foo; bar; end > baz 2>&1",
            ok![while_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(12, 1))]]),
                    false
                ]),
                vec![
                    Redirect::write_to(1, w![normal_word!("baz", loc!(23, 1))], false, loc!(21, 1)),
                    Redirect::copy(1, 2, false, loc!(27, 1)),
                ]
            ]]
        );

        assert_parse!(
            parse_shell_command,
            "until foo; bar; end > baz 2>&1",
            ok![until_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(12, 1))]]),
                    false
                ]),
                vec![
                    Redirect::write_to(1, w![normal_word!("baz", loc!(23, 1))], false, loc!(21, 1)),
                    Redirect::copy(1, 2, false, loc!(27, 1)),
                ]
            ]]
        );

        assert_parse!(
            parse_shell_command,
            "ifconfig",
            ok![simple_command!(vec![w!["ifconfig"]], vec![])]
        );

        assert_parse!(
            parse_shell_command,
            "echo if",
            ok![simple_command!(
                vec![w!["echo"], w![normal_word!("if", loc!(6, 1))]],
                vec![]
            )]
        );

        assert_parse!(
            parse_shell_command,
            "foo > bar 2>&1",
            ok![simple_command!(
                vec![w!["foo"]],
                vec![
                    Redirect::write_to(1, w![normal_word!("bar", loc!(7, 1))], false, loc!(5, 1)),
                    Redirect::copy(1, 2, false, loc!(11, 1)),
                ]
            )]
        );
    }

    #[test]
    fn test_parse_if_statement() {
        assert_parse!(
            parse_if_statement,
            "if foo; then bar; fi",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(14, 1))]]),
                    false
                ])
            ]]
        );
        assert_parse!(
            parse_if_statement,
            "if foo\nthen bar\nfi",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(6, 2))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_if_statement,
            "if foo; bar; fi",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(9, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_if_statement,
            "if foo; bar; end",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(9, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_if_statement,
            "if foo;then if bar;then baz; fi; fi",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec![unit![
                    if_stmt![
                        unit![
                            simple_command!(vec![w![normal_word!("bar", loc!(16, 1))]]),
                            false
                        ],
                        vec!(unit![
                            simple_command!(vec![w![normal_word!("baz", loc!(25, 1))]]),
                            false
                        ])
                    ],
                    false
                ]]
            ]]
        );

        assert_parse!(
            parse_if_statement,
            "if foo;then if bar; baz; end; fi",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec![unit![
                    if_stmt![
                        unit![
                            simple_command!(vec![w![normal_word!("bar", loc!(16, 1))]]),
                            false
                        ],
                        vec!(unit![
                            simple_command!(vec![w![normal_word!("baz", loc!(21, 1))]]),
                            false
                        ])
                    ],
                    false
                ]]
            ]]
        );

        assert_parse!(
            parse_if_statement,
            "if if foo; bar; end; baz; end",
            ok![if_stmt![
                unit![
                    if_stmt![
                        unit![
                            simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                            false
                        ],
                        vec!(unit![
                            simple_command!(vec![w![normal_word!("bar", loc!(12, 1))]]),
                            false
                        ])
                    ],
                    false
                ],
                vec![unit![
                    simple_command!(vec![w![normal_word!("baz", loc!(22, 1))]]),
                    false
                ]]
            ]]
        );

        assert_parse!(
            parse_if_statement,
            "if foo; bar; else baz; fi",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(9, 1))]]),
                    false
                ]),
                vec!(unit![
                    simple_command!(vec![w![normal_word!("baz", loc!(19, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_if_statement,
            "if foo; bar; else baz; end",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(9, 1))]]),
                    false
                ]),
                vec!(unit![
                    simple_command!(vec![w![normal_word!("baz", loc!(19, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_if_statement,
            "if foo; bar; elif baz; foo; fi",
            ok![if_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(4, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(9, 1))]]),
                    false
                ]),
                vec![unit![
                    if_stmt![
                        unit![
                            simple_command!(vec![w![normal_word!("baz", loc!(19, 1))]]),
                            false
                        ],
                        vec!(unit![
                            simple_command!(vec![w![normal_word!("foo", loc!(24, 1))]]),
                            false
                        ])
                    ],
                    false
                ]]
            ]]
        );
    }

    #[test]
    fn test_parse_unless_statement() {
        assert_parse!(
            parse_unless_statement,
            "unless foo; then bar; end",
            ok![unless_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(8, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(18, 1))]]),
                    false
                ])
            ]]
        );
        assert_parse!(
            parse_unless_statement,
            "unless foo\nbar\nend",
            ok![unless_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(8, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(1, 2))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_unless_statement,
            "unless foo; bar; end",
            ok![unless_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(8, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(13, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_unless_statement,
            "unless foo; bar; else baz; end",
            ok![unless_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(8, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(13, 1))]]),
                    false
                ]),
                vec!(unit![
                    simple_command!(vec![w![normal_word!("baz", loc!(23, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_unless_statement,
            "unless foo; then bar; fi",
            Err(ShellError::unexpected_token(Token::fi_keyword(loc!(23, 1))),)
        );
    }

    #[test]
    fn test_parse_while_or_until_statement() {
        assert_parse!(
            parse_while_or_until_statement,
            "while foo; bar; end",
            ok![while_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(12, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "while foo; bar; done",
            ok![while_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(12, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "while foo; do bar; end",
            ok![while_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(15, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "while foo; do bar; done",
            ok![while_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(15, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "while foo\nbar\nend",
            ok![while_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(1, 2))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "until foo; bar; end",
            ok![until_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(12, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "until foo; bar; done",
            ok![until_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(12, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "until foo; do bar; end",
            ok![until_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(15, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "until foo; do bar; done",
            ok![until_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(15, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_while_or_until_statement,
            "until foo\nbar\nend",
            ok![until_stmt![
                unit![
                    simple_command!(vec![w![normal_word!("foo", loc!(7, 1))]]),
                    false
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(1, 2))]]),
                    false
                ])
            ]]
        );
    }

    #[test]
    fn test_parse_for_statement() {
        assert_parse!(
            parse_for_statement,
            "for foo; do bar; done",
            ok![for_stmt![
                Word::new("foo".to_string(), WordKind::Normal, loc!(5, 1)),
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(13, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_for_statement,
            "for foo; do bar; end",
            ok![for_stmt![
                Word::new("foo".to_string(), WordKind::Normal, loc!(5, 1)),
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(13, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_for_statement,
            "for foo; { bar; }",
            ok![for_stmt![
                Word::new("foo".to_string(), WordKind::Normal, loc!(5, 1)),
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(12, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_for_statement,
            "for foo; bar; end",
            ok![for_stmt![
                Word::new("foo".to_string(), WordKind::Normal, loc!(5, 1)),
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(10, 1))]]),
                    false
                ])
            ]]
        );

        assert_parse!(
            parse_for_statement,
            "for foo in a \"b\" 'c'; bar; end",
            ok![for_stmt![
                Word::new("foo".to_string(), WordKind::Normal, loc!(5, 1)),
                vec![
                    w![normal_word!("a", loc!(12, 1))],
                    w![quote_word!("b", loc!(14, 1))],
                    w![literal_word!("c", loc!(18, 1))],
                ],
                vec!(unit![
                    simple_command!(vec![w![normal_word!("bar", loc!(23, 1))]]),
                    false
                ])
            ]]
        );

        /*
        assert_parse!(
            parse_for_statement,
            "for \"foo\"; bar; end",
            Err(ShellError::invalid_identifier(
                "\"foo\"".to_string(),
                loc!(5, 1)
            ),)
        );

        assert_parse!(
            parse_for_statement,
            "for 'foo'; bar; end",
            Err(ShellError::invalid_identifier(
                "'foo'".to_string(),
                loc!(5, 1)
            ),)
        );

        assert_parse!(
            parse_for_statement,
            "for `foo`; bar; end",
            Err(ShellError::invalid_identifier(
                "`foo`".to_string(),
                loc!(5, 1)
            ),)
        );

        assert_parse!(
            parse_for_statement,
            "for $foo; bar; end",
            Err(ShellError::invalid_identifier(
                "$foo".to_string(),
                loc!(5, 1)
            ),)
        );

        assert_parse!(
            parse_for_statement,
            "for ${foo}; bar; end",
            Err(ShellError::invalid_identifier(
                "${foo}".to_string(),
                loc!(5, 1)
            ),)
        );
        */
    }

    #[test]
    fn test_parse_simple_command() {
        assert_parse!(
            parse_simple_command,
            "foobar",
            ok![simple_command!(vec![w!["foobar"]], vec![])]
        );

        assert_parse!(
            parse_simple_command,
            "foo bar",
            ok![simple_command!(
                vec![w!["foo"], w![normal_word!("bar", loc!(5, 1))]],
                vec![]
            )]
        );

        assert_parse!(
            parse_simple_command,
            "foo > bar 2>&1",
            ok![simple_command!(
                vec![w!["foo"]],
                vec![
                    Redirect::write_to(1, w![normal_word!("bar", loc!(7, 1))], false, loc!(5, 1)),
                    Redirect::copy(1, 2, false, loc!(11, 1)),
                ]
            )]
        );

        assert_parse!(
            parse_simple_command,
            "> bar foo",
            ok![simple_command!(
                vec![w![normal_word!("foo", loc!(7, 1))]],
                vec![Redirect::write_to(
                    1,
                    w![normal_word!("bar", loc!(3, 1))],
                    false,
                    loc!(1, 1)
                )]
            )]
        );
    }
}
