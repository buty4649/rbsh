extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_until_command() {
    assert_parse!("until foo; do bar; baz; done" => Ok(vec![until_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            until foo
            do
                bar
                baz
            done
        ") => Ok(vec![until_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("until foo; bar; baz; end" => Ok(vec![until_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            until foo
                bar
                baz
            end
        ") => Ok(vec![until_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("until if foo; then bar; else baz; fi; do if hoge; then fuga; else piyo; fi done" => Ok(vec![until_command!(
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
            until if foo
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
        ") => Ok(vec![until_command!(
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
fn test_until_command_with_redirect() {
    assert_parse!("until foo; do bar; baz; done > hoge" => Ok(vec![until_command!(
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
