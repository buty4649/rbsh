extern crate rbsh_parser;

#[macro_use]
mod common;

use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_connector() {
    assert_parse!("foo && bar && baz" => Ok(vec![and!(
        command!(name: vec![bare!(foo)]),
        and!(
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)])
        )
    )]));
    assert_parse!("foo || bar || baz" => Ok(vec![or!(
        command!(name: vec![bare!(foo)]),
        or!(
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)])
        )
    )]));
    assert_parse!("foo && bar || baz" => Ok(vec![and!(
        command!(name: vec![bare!(foo)]),
        or!(
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)])
        )
    )]));
    assert_parse!("foo || bar && baz" => Ok(vec![or!(
        command!(name: vec![bare!(foo)]),
        and!(
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)])
        )
    )]));

    assert_parse!("if foo && bar && baz; then hoge; fi" => Ok(vec![if_command!(
        body: cond!(
            test: and!(
                command!(name: vec![bare!(foo)]),
                and!(
                    command!(name: vec![bare!(bar)]),
                    command!(name: vec![bare!(baz)])
                )
            ),
            body: vec![
                command!(name: vec![bare!(hoge)])
            ]
        )
    )]));
    assert_parse!("if foo; then bar; fi && hoge" => Ok(vec![and!(
        if_command!(
            body: cond!(
                test: command!(name: vec![bare!(foo)]),
                body: vec![command!(name: vec![bare!(bar)])]
            )
        ),
        command!(name: vec![bare!(hoge)])
    )]));
    assert_parse!("foo && if hoge; then fuga; fi" => Ok(vec![and!(
        command!(name: vec![bare!(foo)]),
        if_command!(
            body: cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![command!(name: vec![bare!(fuga)])]
            )
        )
    )]));
}
