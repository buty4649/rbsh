extern crate rbsh_parser;

#[macro_use]
mod common;

use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_pipe() {
    assert_parse!("foo | bar | baz" => Ok(vec![pipe!(
        command!(name: vec![bare!(foo)]),
        pipe!(
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)])
        )
    )]));
    assert_parse!("foo |& bar |& baz" => Ok(vec![pipe_both!(
        command!(name: vec![bare!(foo)]),
        pipe_both!(
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)])
        )
    )]));
    assert_parse!("foo | bar |& baz" => Ok(vec![pipe!(
        command!(name: vec![bare!(foo)]),
        pipe_both!(
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)])
        )
    )]));
    assert_parse!("foo |& bar | baz" => Ok(vec![pipe_both!(
        command!(name: vec![bare!(foo)]),
        pipe!(
            command!(name: vec![bare!(bar)]),
            command!(name: vec![bare!(baz)])
        )
    )]));

    assert_parse!("foo | bar && hoge" => Ok(vec![and!(
        pipe!(
            command!(name: vec![bare!(foo)]),
            command!(name: vec![bare!(bar)])
        ),
        command!(name: vec![bare!(hoge)])
    )]));

    assert_parse!("foo && hoge | fuga" => Ok(vec![and!(
        command!(name: vec![bare!(foo)]),
        pipe!(
            command!(name: vec![bare!(hoge)]),
            command!(name: vec![bare!(fuga)])
        )
    )]));

    assert_parse!("!foo | bar | baz" => Ok(vec![invert_return!(
        pipe!(
            command!(name: vec![bare!(foo)]),
            pipe!(
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)])
            )
        )
    )]));
    assert_parse!("! foo | bar | baz" => Ok(vec![invert_return!(
        pipe!(
            command!(name: vec![bare!(foo)]),
            pipe!(
                command!(name: vec![bare!(bar)]),
                command!(name: vec![bare!(baz)])
            )
        )
    )]));
    assert_parse!("!foo | bar && !hoge | fuga" => Ok(vec![and!(
        invert_return!(
            pipe!(
                command!(name: vec![bare!(foo)]),
                command!(name: vec![bare!(bar)])
            )
        ),
        invert_return!(
            pipe!(
                command!(name: vec![bare!(hoge)]),
                command!(name: vec![bare!(fuga)])
            )
        )
    )]));
    assert_parse_error!("!foo | !bar");

    assert_parse!("if foo | bar; then hoge; fi" => Ok(vec![if_command!(
        body: cond!(
            test: pipe!(
                command!(name: vec![bare!(foo)]),
                command!(name: vec![bare!(bar)])
            ),
            body: vec![
                command!(name: vec![bare!(hoge)])
            ]
        )
    )]));
    assert_parse!("foo | if hoge | fuga; then piyo; fi" => Ok(vec![pipe!(
        command!(name: vec![bare!(foo)]),
        if_command!(
            body: cond!(
                test: pipe!(
                    command!(name: vec![bare!(hoge)]),
                    command!(name: vec![bare!(fuga)])
                ),
                body: vec![
                    command!(name: vec![bare!(piyo)])
                ]
            )
        )
    )]));
    assert_parse!("if foo | bar; then baz; fi | hoge" => Ok(vec![pipe!(
        if_command!(
            body: cond!(
                test: pipe!(
                    command!(name: vec![bare!(foo)]),
                    command!(name: vec![bare!(bar)])
                ),
                body: vec![
                    command!(name: vec![bare!(baz)])
                ]
            )
        ),
        command!(name: vec![bare!(hoge)])
    )]));

    assert_parse!("!;hoge" => Ok(vec![
        invert_return!(),
        command!(name: vec![bare!(hoge)])
    ]))
}
