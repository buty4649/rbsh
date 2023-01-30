extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_unless_command() {
    assert_parse!("unless foo; then end" => Ok(vec![unless_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![]
        )
    )]));
    assert_parse!("unless foo; then bar; baz; end" => Ok(vec![unless_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            unless foo
            then
                bar
                baz
            end
        ") => Ok(vec![unless_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("unless foo; bar; baz; end" => Ok(vec![unless_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));
    assert_parse!(indoc!("
            unless foo
                bar
                baz
            end
        ") => Ok(vec![unless_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)]),
            ]
        )
    )]));

    assert_parse!("unless foo; bar; else baz; end" => Ok(vec![unless_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![command!(name: vec![bare!(bar)])]
        ),
        else: vec![
            command!(name: vec![bare!(baz)])
        ]
    )]));
    assert_parse!(indoc!("
            unless foo
                bar
            else
                baz
            end
        ") => Ok(vec![unless_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![command!(name: vec![bare!(bar)])]
        ),
        else: vec![
            command!(name: vec![bare!(baz)])
        ]
    )]));
}

#[test]
fn test_unless_command_with_redirect() {
    assert_parse!("unless foo; then bar; end > baz" => Ok(vec![unless_command!(
        body: cond!(
            test: command!(name: vec![bare!(foo)]),
            body: vec![command!(name: vec![bare!(bar)])]
        ),
        redirect: vec![
            redirect_write_to!(1, vec![bare!(baz)], false)
        ]
    )]));
}
