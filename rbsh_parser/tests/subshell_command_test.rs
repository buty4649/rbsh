extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_subshell_command() {
    assert_parse!("(foo; bar; baz)" => Ok(vec![subshell_command!(
        body: vec![
            command!(name: vec![bare!(foo)]),
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ]
    )]));
    assert_parse!(indoc!("
            (
                foo
                bar
                baz
            )
        ") => Ok(vec![subshell_command!(
        body: vec![
            command!(name: vec![bare!(foo)]),
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ]
    )]));
    assert_parse!("(foo; bar; baz;)" => Ok(vec![subshell_command!(
        body: vec![
            command!(name: vec![bare!(foo)]),
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ]
    )]));

    assert_parse!("if (foo) then bar; fi" => Ok(vec![if_command!(
        body: cond!(
            test: subshell_command!(
                body: vec![command!(name: vec![bare!(foo)])]
            ),
            body: vec![
                command!(name: vec!(bare!(bar)))
            ]
        )
    )]));
    assert_parse!(indoc!("
            if (
                foo
            ) then
                bar
            fi
        ") => Ok(vec![if_command!(
        body: cond!(
            test: subshell_command!(
                body: vec![command!(name: vec![bare!(foo)])]
            ),
            body: vec![
                command!(name: vec!(bare!(bar)))
            ]
        )
    )]));

    assert_parse!("((foo ) ;bar;(baz ))" => Ok(vec![subshell_command!(
        body: vec![
            subshell_command!(body: vec![command!(name: vec![bare!(foo)])]),
            command!(name: vec![bare!(bar)]),
            subshell_command!(body: vec![command!(name: vec![bare!(baz)])]),
        ]
    )]));
    assert_parse!(indoc!("
            (
                (
                    foo
                )
                bar
                (
                    baz
                )
            )
        ") => Ok(vec![subshell_command!(
        body: vec![
            subshell_command!(body: vec![command!(name: vec![bare!(foo)])]),
            command!(name: vec![bare!(bar)]),
            subshell_command!(body: vec![command!(name: vec![bare!(baz)])]),
        ]
    )]));

    assert_parse!("({ foo;}; bar; )" => Ok(vec![subshell_command!(
        body: vec![
            group_command!(body: vec![command!(name: vec![bare!(foo)])]),
            command!(name: vec![bare!(bar)]),
        ]
    )]));
    assert_parse!(indoc!("
            (
                {
                    foo
                }
                bar
            )
        ") => Ok(vec![subshell_command!(
        body: vec![
            group_command!(body: vec![command!(name: vec![bare!(foo)])]),
            command!(name: vec![bare!(bar)]),
        ]
    )]));

    assert_parse!("(foo() { bar; }; foo)" => Ok(vec![subshell_command!(
        body: vec![
            function_command!(ident: "foo", body: group_command!(
                body: vec![
                    command!(name: vec![bare!(bar)])
                ]
            )),
            command!(name: vec![bare!(foo)]),
        ]
    )]));
    assert_parse!(indoc!("
            (
                foo() {
                    bar
                }
                foo
            )
        ") => Ok(vec![subshell_command!(
        body: vec![
            function_command!(ident: "foo", body: group_command!(
                body: vec![
                    command!(name: vec![bare!(bar)])
                ]
            )),
            command!(name: vec![bare!(foo)]),
        ]
    )]));
}

#[test]
fn test_subshell_command_with_redirect() {
    assert_parse!("(foo; bar; baz) > hoge" => Ok(vec![subshell_command!(
        body: vec![
            command!(name: vec![bare!(foo)]),
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ],
        redirect: vec![
            redirect_write_to!(1, vec![bare!(hoge)], false),
        ]
    )]));
}
