extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_select_command() {
    assert_parse!("select foo; do bar; baz; done" => Ok(vec![select_command!(
        ident: "foo",
        body: vec![
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ]
    )]));
    assert_parse!(indoc!("
            select foo
            do
                bar
                baz
            done
        ") => Ok(vec![select_command!(
        ident: "foo",
        body: vec![
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)]),
        ]
    )]));

    assert_parse!("select foo in bar $baz; do hoge; fuga; done" => Ok(vec![select_command!(
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
            select foo in bar $baz
            do
                hoge
                fuga
            done
        ") => Ok(vec![select_command!(
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

    assert_parse!("select foo in bar baz; do hoge; fuga; end" => Ok(vec![select_command!(
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
            select foo in bar baz
            do
                hoge
                fuga
            end
        ") => Ok(vec![select_command!(
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

    assert_parse_error!("select $foo; do bar; baz; done");
    assert_parse_error!("select foo in bar baz; hoge; fuga; done");
    assert_parse_error!("select foo; bar; baz; done");
}

#[test]
fn test_select_command_with_redirect() {
    assert_parse!("select foo; do bar; baz; done > hoge" => Ok(vec![select_command!(
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
