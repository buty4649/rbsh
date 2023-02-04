extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_if_command() {
    assert_parse!("if foo; then fi" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![]
        )
    )]));
    assert_parse!("if foo; then bar; baz; fi" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            if foo
            then
                bar
                baz
            fi
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("if foo; then bar; baz; elif hoge; then fuga; piyo; elif spam; then ham; egg; fi" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![
            cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![
                    command!(name: vec![bare!(fuga)]),
                    command!(name: vec![bare!(piyo)]),
                ]
            ),
            cond!(
                test: command!(name: vec![bare!(spam)]),
                body: vec![
                    command!(name: vec![bare!(ham)]),
                    command!(name: vec![bare!(egg)]),
                ]
            ),
        ]
    )]));

    assert_parse!(indoc!("
        if foo
        then
            bar
            baz
        elif hoge
        then
            fuga
            piyo
        elif spam
        then
            ham
            egg
        fi
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![
            cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![
                    command!(name: vec![bare!(fuga)]),
                    command!(name: vec![bare!(piyo)]),
                ]
            ),
            cond!(
                test: command!(name: vec![bare!(spam)]),
                body: vec![
                    command!(name: vec![bare!(ham)]),
                    command!(name: vec![bare!(egg)]),
                ]
            ),
        ]
    )]));

    assert_parse!("if foo; then bar; baz; else hoge; fuga; fi" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        else: vec![
            command!(name: vec![bare!(hoge)]),
            command!(name: vec![bare!(fuga)]),
        ]
    )]));
    assert_parse!(indoc!("
            if foo
            then
                bar
                baz
            else
                hoge
                fuga
            fi
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        else: vec![
            command!(name: vec![bare!(hoge)]),
            command!(name: vec![bare!(fuga)]),
        ]
    )]));

    assert_parse!("if foo; then bar; baz; elif hoge; then fuga; piyo; else spam; ham; fi" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![
            cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![
                    command!(name: vec![bare!(fuga)]),
                    command!(name: vec![bare!(piyo)]),
                ]
            ),
        ],
        else: vec![
            command!(name: vec![bare!(spam)]),
            command!(name: vec![bare!(ham)]),
        ]
    )]));
    assert_parse!(indoc!("
        if foo
        then
            bar
            baz
        elif hoge
        then
            fuga
            piyo
        else
            spam
            ham
        fi
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![
            cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![
                    command!(name: vec![bare!(fuga)]),
                    command!(name: vec![bare!(piyo)]),
                ]
            ),
        ],
        else: vec![
            command!(name: vec![bare!(spam)]),
            command!(name: vec![bare!(ham)]),
        ]
    )]));

    assert_parse!("if if foo; then bar; else baz; fi; then if hoge; then fuga; else piyo; fi; fi" => Ok(vec![if_command!(
        body: cond!(
            test: if_command!(
                body: cond!(
                    test: command!(name: vec![bare!(foo)]),
                    body: vec![command!(name: vec![bare!(bar)])]
                ),
                else: vec![command!(name: vec![bare!(baz)])]
            ),
            body: vec![
                if_command!(
                    body: cond!(
                        test: command!(name: vec![bare!(hoge)]),
                        body: vec![command!(name: vec![bare!(fuga)])]
                    ),
                    else: vec![command!(name: vec![bare!(piyo)])]
                )
            ]
        )
    )]));

    assert_parse!(indoc!("
            if
                if foo
                then
                    bar
                else
                    baz
                fi
            then
                if hoge
                then
                    fuga
                else
                    piyo
                fi
            fi
        ") => Ok(vec![if_command!(
        body: cond!(
            test: if_command!(
                body: cond!(
                    test: command!(name: vec![bare!(foo)]),
                    body: vec![command!(name: vec![bare!(bar)])]
                ),
                else: vec![command!(name: vec![bare!(baz)])]
            ),
            body: vec![
                if_command!(
                    body: cond!(
                        test: command!(name: vec![bare!(hoge)]),
                        body: vec![command!(name: vec![bare!(fuga)])]
                    ),
                    else: vec![command!(name: vec![bare!(piyo)])]
                )
            ]
        )
    )]));

    assert_parse_error!("if foo; bar; baz fi");
    assert_parse_error!("if foo then bar; baz fi");
    assert_parse_error!("if foo then bar; baz;");
    assert_parse_error!("if foo; then bar; baz fi");
    assert_parse_error!("if foo; then bar; elif hoge; fuga; fi");
    assert_parse_error!("if foo; then bar; elif hoge; then fuga; end");
}

#[test]
fn test_rubyish_if_command() {
    assert_parse!("if foo; end" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![]
        )
    )]));
    assert_parse!("if foo; then bar; baz; end" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            if foo; then
                bar
                baz
            end
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("if foo; bar; baz; end" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            if foo
               bar
               baz
            end
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("if foo; bar; baz; elsif hoge; then fuga; piyo; end" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![cond!(
            test: command!(name: vec![bare!(hoge)]),
            body: vec![
                command!(name: vec![bare!(fuga)]),
                command!(name: vec![bare!(piyo)]),
            ]
        )]
    )]));
    assert_parse!(indoc!("
            if foo
                bar
                baz
            elsif hoge
            then
                fuga
                piyo
            end
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![cond!(
            test: command!(name: vec![bare!(hoge)]),
            body: vec![
                command!(name: vec![bare!(fuga)]),
                command!(name: vec![bare!(piyo)]),
            ]
        )]
    )]));

    assert_parse!("if foo; bar; baz; elsif hoge; fuga; piyo; end" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![cond!(
            test: command!(name: vec![bare!(hoge)]),
            body: vec![
                command!(name: vec![bare!(fuga)]),
                command!(name: vec![bare!(piyo)]),
            ]
        )]
    )]));
    assert_parse!(indoc!("
            if foo
                bar
                baz
            elsif hoge
               fuga
               piyo
            end
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![cond!(
            test: command!(name: vec![bare!(hoge)]),
            body: vec![
                command!(name: vec![bare!(fuga)]),
                command!(name: vec![bare!(piyo)]),
            ]
        )]
    )]));

    assert_parse!("if foo; bar; baz; elsif hoge; fuga; piyo; else spam; ham; end" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![cond!(
            test: command!(name: vec![bare!(hoge)]),
            body: vec![
                command!(name: vec![bare!(fuga)]),
                command!(name: vec![bare!(piyo)]),
            ]
        )],
        else: vec![
            command!(name: vec![bare!(spam)]),
            command!(name: vec![bare!(ham)]),
        ]
    )]));
    assert_parse!(indoc!("
            if foo
                bar
                baz
            elsif hoge
                fuga
                piyo
            else
                spam
                ham
            end
        ") => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        elif: vec![cond!(
            test: command!(name: vec![bare!(hoge)]),
            body: vec![
                command!(name: vec![bare!(fuga)]),
                command!(name: vec![bare!(piyo)]),
            ]
        )],
        else: vec![
            command!(name: vec![bare!(spam)]),
            command!(name: vec![bare!(ham)]),
        ]
    )]));

    assert_parse!("if if foo; bar; else baz; end; if hoge; fuga; else piyo; end; end" => Ok(vec![if_command!(
        body: cond!(
            test: if_command!(
                body: cond!(
                    test: command!(name: vec![bare!(foo)]),
                    body: vec![command!(name: vec![bare!(bar)])]
                ),
                else: vec![command!(name: vec![bare!(baz)])]
            ),
            body: vec![
                if_command!(
                    body: cond!(
                        test: command!(name: vec![bare!(hoge)]),
                        body: vec![command!(name: vec![bare!(fuga)])]
                    ),
                    else: vec![command!(name: vec![bare!(piyo)])]
                )
            ]
        )
    )]));
    assert_parse!(indoc!("
            if
                if foo
                    bar
                else
                    baz
                end
                if hoge
                    fuga
                else
                    piyo
                end
            end
        ") => Ok(vec![if_command!(
        body: cond!(
            test: if_command!(
                body: cond!(
                    test: command!(name: vec![bare!(foo)]),
                    body: vec![command!(name: vec![bare!(bar)])]
                ),
                else: vec![command!(name: vec![bare!(baz)])]
            ),
            body: vec![
                if_command!(
                    body: cond!(
                        test: command!(name: vec![bare!(hoge)]),
                        body: vec![command!(name: vec![bare!(fuga)])]
                    ),
                    else: vec![command!(name: vec![bare!(piyo)])]
                )
            ]
        )
    )]));
}

#[test]
fn test_if_command_with_redirect() {
    assert_parse!("if foo; then bar; fi > baz" => Ok(vec![if_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![command!(name: vec![bare!(bar)])]
        ),
        redirect: vec![
            redirect_write_to!(1, vec![bare!(baz)], false)
        ]
    )]));
    assert_parse_error!("> foo if bar; then baz; fi");
}
