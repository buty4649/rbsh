extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_function_command() {
    assert_parse!("foo(){ bar; }" => Ok(vec![function_command!(
        ident: "foo",
        body: group_command!(
            body: vec![command!(name: vec![bare!(bar)])]
        )
    )]));
    assert_parse!(indoc!("
            foo()
            {
                bar
            }
        ") => Ok(vec![function_command!(
        ident: "foo",
        body: group_command!(
            body: vec![command!(name: vec![bare!(bar)])]
        )
    )]));

    assert_parse!("function foo(){ bar; }" => Ok(vec![function_command!(
        ident: "foo",
        body: group_command!(
            body: vec![command!(name: vec![bare!(bar)])]
        )
    )]));
    assert_parse!(indoc!("
            function foo()
            {
                bar
            }
        ") => Ok(vec![function_command!(
        ident: "foo",
        body: group_command!(
            body: vec![command!(name: vec![bare!(bar)])]
        )
    )]));

    assert_parse!(indoc!("
            function foo
            {
                bar
            }
        ") => Ok(vec![function_command!(
        ident: "foo",
        body: group_command!(
            body: vec![command!(name: vec![bare!(bar)])]
        )
    )]));

    assert_parse!("foo() if hoge; then fuga; fi" => Ok(vec![function_command!(
        ident: "foo",
        body: if_command!(
            body: cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![command!(name: vec![bare!(fuga)])]
            )
        )
    )]));
    assert_parse!("foo() unless hoge; then fuga; end" => Ok(vec![function_command!(
        ident: "foo",
        body: unless_command!(
            body: cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![command!(name: vec![bare!(fuga)])]
            )
        )
    )]));
    assert_parse!("foo() while hoge; do fuga; done" => Ok(vec![function_command!(
        ident: "foo",
        body: while_command!(
            body: cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![command!(name: vec![bare!(fuga)])]
            )
        )
    )]));
    assert_parse!("foo() until hoge; do fuga; done" => Ok(vec![function_command!(
        ident: "foo",
        body: until_command!(
            body: cond!(
                test: command!(name: vec![bare!(hoge)]),
                body: vec![command!(name: vec![bare!(fuga)])]
            )
        )
    )]));
    assert_parse!("foo() for hoge; do fuga; done" => Ok(vec![function_command!(
        ident: "foo",
        body: for_command!(
            ident: "hoge",
            body: vec![command!(name: vec![bare!(fuga)])]
        )
    )]));
    assert_parse!("foo() select hoge; do fuga; done" => Ok(vec![function_command!(
        ident: "foo",
        body: select_command!(
            ident: "hoge",
            body: vec![command!(name: vec![bare!(fuga)])]
        )
    )]));
    assert_parse!("foo() case hoge in fuga) piyo;; esac" => Ok(vec![function_command!(
        ident: "foo",
        body: case_command!(
            word: vec![bare!(hoge)],
            pattern: vec![case_pattern!(
                pattern: vec![vec![bare!(fuga)]],
                body: vec![command!(name: vec![bare!(piyo)])],
                next_action: CasePatternNextAction::End
            )]
        )
    )]));
    assert_parse!("foo()(bar )" => Ok(vec![function_command!(
        ident: "foo",
        body: subshell_command!(
            body: vec![command!(name: vec![bare!(bar)])]
        )
    )]));

    assert_parse!("if foo(){ bar; }; then hoge; fi" => Ok(vec![if_command!(
        body: cond!(
            test: function_command!(
                ident: "foo",
                body: group_command!(
                    body: vec![command!(name: vec![bare!(bar)])]
                )
            ),
            body: vec![command!(name: vec![bare!(hoge)])]
        )
    )]));
    assert_parse!(indoc!("
            if foo()
               {
                    bar
                }
            then hoge
            fi
        ") => Ok(vec![if_command!(
        body: cond!(
            test: function_command!(
                ident: "foo",
                body: group_command!(
                    body: vec![command!(name: vec![bare!(bar)])]
                )
            ),
            body: vec![command!(name: vec![bare!(hoge)])]
        )
    )]));

    assert_parse_error!("foo() bar() { baz; }");
}

#[test]
fn test_function_command_with_redirect() {
    assert_parse!("foo(){ bar; } > hoge" => Ok(vec![function_command!(
        ident: "foo",
        body: group_command!(
            body: vec![command!(name: vec![bare!(bar)])],
            redirect: vec![
                redirect_write_to!(1, vec![bare!(hoge)], false)
            ]
        )
    )]));
}
