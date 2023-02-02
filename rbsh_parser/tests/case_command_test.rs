extern crate rbsh_parser;

#[macro_use]
mod common;

use indoc::indoc;
use pretty_assertions::assert_eq;
use rbsh_parser::ast::*;
use rbsh_parser::parse;

#[test]
fn test_case_command() {
    assert_parse!(r#"case foo"bar"$baz in esac"# => Ok(vec![case_command!(
        word: vec![bare!(foo), quote!(bare!(bar)), param_sub!(baz)]
    )]));
    assert_parse!(indoc!(r#"
            case foo"bar"$baz in
            esac
        "#) => Ok(vec![case_command!(
        word: vec![bare!(foo), quote!(bare!(bar)), param_sub!(baz)]
    )]));

    assert_parse!("case foo in (bar) baz ;; (hoge) fuga; piyo ;; esac" => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            ),
            case_pattern!(
                pattern: vec![vec![bare!(hoge)]],
                body: vec![
                    command!(name: vec![bare!(fuga)]),
                    command!(name: vec![bare!(piyo)]),
                ],
                next_action: CasePatternNextAction::End
            )
        ]
    )]));
    assert_parse!(indoc!("
            case foo in
                (bar)
                    baz
                    ;;
                (hoge)
                    fuga; piyo
                    ;;
            esac
        ") => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            ),
            case_pattern!(
                pattern: vec![vec![bare!(hoge)]],
                body: vec![
                    command!(name: vec![bare!(fuga)]),
                    command!(name: vec![bare!(piyo)]),
                ],
                next_action: CasePatternNextAction::End
            )
        ]
    )]));

    assert_parse!("case foo in (bar) baz; esac" => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            )
        ]
    )]));
    assert_parse!(indoc!("
            case foo in
                (bar) baz;
            esac
        ") => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            ),
        ]
    )]));

    assert_parse!("case foo in bar) baz; esac" => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            )
        ]
    )]));
    assert_parse!(indoc!("
            case foo in
                bar) baz;
            esac
        ") => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            ),
        ]
    )]));

    assert_parse!("case foo in bar | baz) hoge; fuga ; esac" => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![
                    vec![bare!(bar)],
                    vec![bare!(baz)]
                ],
                body: vec![
                    command!(name: vec![bare!(hoge)]),
                    command!(name: vec![bare!(fuga)]),
                ],
                next_action: CasePatternNextAction::End
            )
        ]
    )]));
    assert_parse!(indoc!("
            case foo in
                bar | baz)
                    hoge
                    fuga
            esac
        ") => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![
                    vec![bare!(bar)],
                    vec![bare!(baz)]
                ],
                body: vec![
                    command!(name: vec![bare!(hoge)]),
                    command!(name: vec![bare!(fuga)]),
                ],
                next_action: CasePatternNextAction::End
            )
        ]
    )]));

    assert_parse!("case foo in bar) baz ;& hoge) fuga ;;& esac" => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::FallThrough
            ),
            case_pattern!(
                pattern: vec![vec![bare!(hoge)]],
                body: vec![
                    command!(name: vec![bare!(fuga)]),
                ],
                next_action: CasePatternNextAction::TestNext
            )
        ]
    )]));
    assert_parse!(indoc!("
            case foo in
                bar) baz ;&
                hoge) fuga ;;&
            esac
        ") => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::FallThrough
            ),
            case_pattern!(
                pattern: vec![vec![bare!(hoge)]],
                body: vec![
                    command!(name: vec![bare!(fuga)]),
                ],
                next_action: CasePatternNextAction::TestNext
            )
        ]
    )]));

    assert_parse!("case foo when bar then baz; end" => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            ),
        ]
    )]));
    assert_parse!(indoc!("
            case foo
            when bar
                baz
            end
        ") => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            ),
        ]
    )]));

    assert_parse_error!("foo;;");
    assert_parse_error!("case foo bar) baz; esac");
    assert_parse_error!("case foo in bar) baz esac");
    assert_parse_error!("case foo when bar baz; esac");
}

#[test]
fn test_select_command_with_redirect() {
    assert_parse!("case foo in bar) baz; esac > hoge" => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            )
        ],
        redirect: vec![
            redirect_write_to!(1, vec![bare!(hoge)], false),
        ]
    )]));
    assert_parse!(indoc!("
            case foo in
                bar) baz;
            esac > hoge
        ") => Ok(vec![case_command!(
        word: vec![bare!(foo)],
        pattern: vec![
            case_pattern!(
                pattern: vec![vec![bare!(bar)]],
                body: vec![
                    command!(name: vec![bare!(baz)]),
                ],
                next_action: CasePatternNextAction::End
            ),
        ],
        redirect: vec![
            redirect_write_to!(1, vec![bare!(hoge)], false),
        ]
    )]));
}
