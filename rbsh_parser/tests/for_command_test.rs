extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_for_command() {
    assert_parse!("for foo; do bar; baz; done" => Ok(vec![for_command!(
        ident: "foo",
        body: vec![
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ]
    )]));
    assert_parse!(indoc!("
            for foo
            do
                bar
                baz
            done
        ") => Ok(vec![for_command!(
        ident: "foo",
        body: vec![
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ]
    )]));

    assert_parse!("for foo in bar $baz; do hoge; fuga; done" => Ok(vec![for_command!(
        ident: "foo",
        subject: vec![
            vec![bare!(bar)],
            vec![param_sub!(baz)],
        ],
        body: vec![
            command!(name: vec![bare!(hoge)]),
            command!(name: vec![bare!(fuga)]),
        ]
    )]));
    assert_parse!(indoc!("
            for foo in bar $baz
            do
                hoge
                fuga
            done
        ") => Ok(vec![for_command!(
        ident: "foo",
        subject: vec![
            vec![bare!(bar)],
            vec![param_sub!(baz)],
        ],
        body: vec![
            command!(name: vec![bare!(hoge)]),
            command!(name: vec![bare!(fuga)]),
        ]
    )]));

    assert_parse!("for foo in bar baz; do hoge; fuga; end" => Ok(vec![for_command!(
        ident: "foo",
        subject: vec![
            vec![bare!(bar)],
            vec![bare!(baz)],
        ],
        body: vec![
            command!(name: vec![bare!(hoge)]),
            command!(name: vec![bare!(fuga)]),
        ]
    )]));
    assert_parse!(indoc!("
            for foo in bar baz
            do
                hoge
                fuga
            end
        ") => Ok(vec![for_command!(
        ident: "foo",
        subject: vec![
            vec![bare!(bar)],
            vec![bare!(baz)],
        ],
        body: vec![
            command!(name: vec![bare!(hoge)]),
            command!(name: vec![bare!(fuga)]),
        ]
    )]));

    assert_parse_error!("for $foo; do bar; baz; done");
    assert_parse_error!("for foo in bar baz; hoge; fuga; done");
    assert_parse_error!("for foo; bar; baz; done");
}

#[test]
fn test_while_command_with_redirect() {
    assert_parse!("for foo; do bar; baz; done > hoge" => Ok(vec![for_command!(
        ident: "foo",
        body: vec![
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ],
        redirect: vec![
            redirect_write_to!(1, vec![bare!(hoge)], false)
        ]
    )]));
}
