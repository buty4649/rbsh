extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_while_command() {
    assert_parse!("while foo; do bar; baz; done" => Ok(vec![while_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            while foo
            do
                bar
                baz
            done
        ") => Ok(vec![while_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("while foo; bar; baz; end" => Ok(vec![while_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            while foo
                bar
                baz
            end
        ") => Ok(vec![while_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("while if foo; then bar; else baz; fi; do if hoge; then fuga; else piyo; fi done" => Ok(vec![while_command!(
        body: cond!(
            test: if_command!(
                body: cond!(
                    test: command!(name: vec![bare!(foo)]),
                    body: vec![command!(name: vec!(bare!(bar)))]
                ),
                else: vec![command!(name: vec![bare!(baz)])]
            ),
            body: vec![
                if_command!(
                    body: cond!(
                        test: command!(name: vec![bare!(hoge)]),
                        body: vec![command!(name: vec!(bare!(fuga)))]
                    ),
                    else: vec![command!(name: vec![bare!(piyo)])]
                ),
            ]
        )
    )]));
    assert_parse!(indoc!("
            while if foo
                then
                    bar
                else
                    baz
                fi
            do
                if
                    hoge
                then
                    fuga
                else
                    piyo
                fi
            done
        ") => Ok(vec![while_command!(
        body: cond!(
            test: if_command!(
                body: cond!(
                    test: command!(name: vec![bare!(foo)]),
                    body: vec![command!(name: vec!(bare!(bar)))]
                ),
                else: vec![command!(name: vec![bare!(baz)])]
            ),
            body: vec![
                if_command!(
                    body: cond!(
                        test: command!(name: vec![bare!(hoge)]),
                        body: vec![command!(name: vec!(bare!(fuga)))]
                    ),
                    else: vec![command!(name: vec![bare!(piyo)])]
                ),
            ]
        )
    )]));
}

#[test]
fn test_while_command_with_redirect() {
    assert_parse!("while foo; do bar; baz; done > hoge" => Ok(vec![while_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        ),
        redirect: vec![
            redirect_write_to!(1, vec![bare!(hoge)], false)
        ]
    )]));
}
