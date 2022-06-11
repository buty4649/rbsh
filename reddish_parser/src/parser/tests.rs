#[cfg(test)]
mod test {
    use super::*;
    use crate::{location, Location, Token};
    use indoc::indoc;

    macro_rules! assert_parse {
        ($s: tt, $offset: expr, ok![ $unit:expr, $ignore_history: expr ]) => {
            assert_eq!(
                parse_command_line(indoc! {$s}, $offset),
                Ok(($unit, $ignore_history))
            )
        };
        ($s: tt, ok![ $unit:expr, $ignore_history: expr ]) => {
            assert_parse!($s, 0, ok![$unit, $ignore_history])
        };
        ($s: tt, ok![ $unit:expr ]) => {
            assert_parse!($s, 0, ok![$unit, false])
        };
        ($s: tt, err![ $e:expr ]) => {
            assert_eq!(parse_command_line($s, 0), Err($e))
        };
    }

    macro_rules! simple_command {
        ($s:expr, $loc:expr, $r:expr, $b:expr) => {{
            let command = if $s.is_empty() {
                vec![]
            } else {
                vec![vec![Word::normal($s, $loc)]]
            };
            Unit::new(
                UnitKind::SimpleCommand {
                    command,
                    redirect: $r,
                },
                $b,
            )
        }};
        ($s:expr, $loc:expr, $r:expr) => {
            simple_command!($s, $loc, $r, false)
        };
        ($s:expr, $loc:expr) => {
            simple_command!($s, $loc, None)
        };
        ($s:expr) => {
            simple_command!($s, location!(1))
        };
    }

    macro_rules! connecter_pipe {
        ($left:expr, $right:expr, $both:expr, $background:expr) => {
            Unit::new(
                UnitKind::Pipe {
                    left: Box::new($left),
                    right: Box::new($right),
                    both: $both,
                },
                $background,
            )
        };
        ($left:expr, $right:expr, $both:expr) => {
            connecter_pipe!($left, $right, $both, false)
        };

        ($left:expr, $right:expr) => {
            connecter_pipe!($left, $right, false)
        };
    }

    macro_rules! connecter_pipe_both {
        ($left:expr, $right:expr) => {
            connecter_pipe!($left, $right, true)
        };
    }

    macro_rules! connecter_and {
        ($left:expr, $right:expr, $background:expr) => {
            Unit::new(
                UnitKind::Connecter {
                    left: Box::new($left),
                    right: Box::new($right),
                    kind: ConnecterKind::And,
                },
                $background,
            )
        };
        ($left:expr, $right:expr) => {
            connecter_and!($left, $right, false)
        };
    }

    macro_rules! connecter_or {
        ($left:expr, $right:expr, $background:expr) => {
            Unit::new(
                UnitKind::Connecter {
                    left: Box::new($left),
                    right: Box::new($right),
                    kind: ConnecterKind::Or,
                },
                $background,
            )
        };
        ($left:expr, $right:expr) => {
            connecter_or!($left, $right, false)
        };
    }

    macro_rules! if_statement {
        ($condition:expr, $true_case:expr, $false_case:expr, $redirect:expr, $background:expr) => {
            Unit::new(
                UnitKind::If {
                    condition: Box::new($condition),
                    true_case: $true_case,
                    false_case: $false_case,
                    redirect: $redirect,
                },
                $background,
            )
        };
        ($condition:expr, $true_case:expr, $false_case:expr, $redirect:expr) => {
            if_statement!($condition, $true_case, $false_case, $redirect, false)
        };
        ($condition:expr, $true_case:expr, $false_case:expr) => {
            if_statement!($condition, $true_case, $false_case, None)
        };
        ($condition:expr, $true_case:expr) => {
            if_statement!($condition, $true_case, None, None, false)
        };
    }

    macro_rules! unless_statement {
        ($condition:expr, $false_case:expr, $true_case:expr, $redirect:expr, $background:expr) => {
            Unit::new(
                UnitKind::Unless {
                    condition: Box::new($condition),
                    false_case: $false_case,
                    true_case: $true_case,
                    redirect: $redirect,
                },
                $background,
            )
        };
        ($condition:expr, $false_case:expr, $true_case:expr, $redirect:expr) => {
            unless_statement!($condition, $false_case, $true_case, $redirect, false)
        };
        ($condition:expr, $false_case:expr, $true_case:expr) => {
            unless_statement!($condition, $false_case, $true_case, None)
        };
        ($condition:expr, $false_case:expr) => {
            unless_statement!($condition, $false_case, None, None, false)
        };
    }

    macro_rules! while_statement {
        ($condition:expr, $command:expr, $redirect:expr, $background:expr) => {
            Unit::new(
                UnitKind::While {
                    condition: Box::new($condition),
                    command: $command,
                    redirect: $redirect,
                },
                $background,
            )
        };
        ($condition:expr, $command:expr, $redirect:expr) => {
            while_statement!($condition, $command, $redirect, false)
        };
        ($condition:expr, $command:expr) => {
            while_statement!($condition, $command, None)
        };
    }

    macro_rules! until_statement {
        ($condition:expr, $command:expr, $redirect:expr, $background:expr) => {
            Unit::new(
                UnitKind::Until {
                    condition: Box::new($condition),
                    command: $command,
                    redirect: $redirect,
                },
                $background,
            )
        };
        ($condition:expr, $command:expr, $redirect:expr) => {
            until_statement!($condition, $command, $redirect, false)
        };
        ($condition:expr, $command:expr) => {
            until_statement!($condition, $command, None)
        };
    }

    macro_rules! for_statement {
        ($identifier:expr, $list:expr, $command:expr, $redirect:expr, $background:expr) => {
            Unit::new(
                UnitKind::For {
                    identifier: $identifier,
                    list: $list,
                    command: $command,
                    redirect: $redirect,
                },
                $background,
            )
        };
        ($identifier:expr, $list:expr, $command:expr, $redirect:expr) => {
            for_statement!($identifier, $list, $command, $redirect, false)
        };
        ($identifier:expr, $list:expr, $command:expr) => {
            for_statement!($identifier, $list, $command, None)
        };
        ($identifier:expr, $command:expr) => {
            for_statement!($identifier, None, $command)
        };
    }

    macro_rules! redirect {
        ($($name:ident($($e:expr),+)),+) => {
            Some(vec![$(redirect_inner!($name, $($e),+),)+])
        }
    }

    macro_rules! redirect_inner {
        (write_to, $s:expr, $loc:expr) => {{
            let l = Location::from_offset(&$loc, 2, 0);
            Redirect::write_to(1, vec![Word::normal($s, l)], false, $loc)
        }};
        (copy, $loc:expr) => {
            Redirect::copy(1, 2, false, $loc)
        };
    }

    #[test]
    fn simple_command() {
        assert_parse!(
            " foo",
            ok![vec![simple_command!("foo", location!(2))], true]
        );

        assert_parse!("foo", ok![vec![simple_command!("foo")]]);
        assert_parse!("foo;", ok![vec![simple_command!("foo")]]);
        assert_parse!("foo ;", ok![vec![simple_command!("foo")]]);
        assert_parse!("foo\n", ok![vec![simple_command!("foo")]]);
        assert_parse!(
            "foo;bar",
            ok![vec![
                simple_command!("foo"),
                simple_command!("bar", location!(5))
            ]]
        );
        assert_parse!(
            "foo ;bar",
            ok![vec![
                simple_command!("foo"),
                simple_command!("bar", location!(6))
            ]]
        );
        assert_parse!(
            "foo ; bar",
            ok![vec![
                simple_command!("foo"),
                simple_command!("bar", location!(7))
            ]]
        );
        assert_parse!(
            "foo\nbar",
            ok![vec![
                simple_command!("foo"),
                simple_command!("bar", location!(1, 2))
            ]]
        );

        assert_parse!(
            "foo&",
            ok![vec![simple_command!("foo", location!(), None, true)]]
        );
        assert_parse!(
            "foo &",
            ok![vec![simple_command!("foo", location!(), None, true)]]
        );
        assert_parse!(
            "foo&bar",
            ok![vec![
                simple_command!("foo", location!(), None, true),
                simple_command!("bar", location!(5))
            ]]
        );

        assert_parse!(
            "foo;bar;baz",
            ok![vec![
                simple_command!("foo"),
                simple_command!("bar", location!(5)),
                simple_command!("baz", location!(9))
            ]]
        );

        assert_parse!(
            "> foo",
            ok![vec![simple_command!(
                "",
                location!(),
                redirect![write_to("foo", location!(1))]
            )]]
        );

        assert_parse!(
            "foo> bar",
            ok![vec![simple_command!(
                "foo",
                location!(),
                redirect![write_to("bar", location!(4))]
            )]]
        );

        assert_parse!(
            "foo > bar",
            ok![vec![simple_command!(
                "foo",
                location!(),
                redirect![write_to("bar", location!(5))]
            )]]
        );

        assert_parse!(
            "> bar foo",
            ok![vec![simple_command!(
                "foo",
                location!(7),
                redirect![write_to("bar", location!(1))]
            )]]
        );

        assert_parse!(
            "> bar foo 2>&1",
            ok![vec![simple_command!(
                "foo",
                location!(7),
                redirect![write_to("bar", location!(1)), copy(location!(11))]
            )]]
        );

        assert_parse!(
            "foo > bar;baz > qux",
            ok![vec![
                simple_command!("foo", location!(), redirect![write_to("bar", location!(5))]),
                simple_command!(
                    "baz",
                    location!(11),
                    redirect![write_to("qux", location!(15))]
                )
            ]]
        )
    }

    #[test]
    fn connecter_pipe() {
        assert_parse!(
            "foo|bar",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                simple_command!("bar", location!(5))
            )]]
        );

        assert_parse!("foo|", err![Error::eof(location!(5))]);
        assert_parse!(
            "foo&|bar",
            err![Error::unexpected_token(&Token::pipe(location!(5)))]
        );
        assert_parse!(
            "foo| &",
            err![Error::unexpected_token(&Token::background(location!(6)))]
        );

        assert_parse!(
            "foo|bar&",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                simple_command!("bar", location!(5), None, true)
            )]]
        );
        assert_parse!(
            "foo > bar|baz",
            ok![vec![connecter_pipe!(
                simple_command!("foo", location!(), redirect![write_to("bar", location!(5))]),
                simple_command!("baz", location!(11))
            )]]
        );
        assert_parse!(
            "foo|bar > baz",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                simple_command!(
                    "bar",
                    location!(5),
                    redirect![write_to("baz", location!(9))]
                )
            )]]
        );

        assert_parse!(
            "foo|\nbar",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                simple_command!("bar", location!(1, 2))
            )]]
        );

        assert_parse!(
            "foo|bar|baz",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                connecter_pipe!(
                    simple_command!("bar", location!(5)),
                    simple_command!("baz", location!(9))
                )
            )]]
        );
        assert_parse!(
            "foo|bar|baz&",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                connecter_pipe!(
                    simple_command!("bar", location!(5)),
                    simple_command!("baz", location!(9), None, true)
                )
            )]]
        );
        assert_parse!(
            "foo|bar > baz|qux",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                connecter_pipe!(
                    simple_command!(
                        "bar",
                        location!(5),
                        redirect![write_to("baz", location!(9))]
                    ),
                    simple_command!("qux", location!(15))
                )
            )]]
        );

        assert_parse!(
            "foo|bar|&baz",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                connecter_pipe_both!(
                    simple_command!("bar", location!(5)),
                    simple_command!("baz", location!(10))
                )
            )]]
        );
        assert_parse!(
            "foo|bar&&baz",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                connecter_and!(
                    simple_command!("bar", location!(5)),
                    simple_command!("baz", location!(10))
                )
            )]]
        );
        assert_parse!(
            "foo|bar||baz",
            ok![vec![connecter_pipe!(
                simple_command!("foo"),
                connecter_or!(
                    simple_command!("bar", location!(5)),
                    simple_command!("baz", location!(10))
                )
            )]]
        );
    }

    #[test]
    fn connecter_pipe_both() {
        assert_parse!(
            "foo|&bar",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                simple_command!("bar", location!(6))
            )]]
        );

        assert_parse!("foo|&", err![Error::eof(location!(6))]);
        assert_parse!(
            "foo&|&bar",
            err![Error::unexpected_token(&Token::pipe_both(location!(5)))]
        );

        assert_parse!(
            "foo|&bar&",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                simple_command!("bar", location!(6), None, true)
            )]]
        );

        assert_parse!(
            "foo > bar|& baz",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo", location!(), redirect![write_to("bar", location!(5))]),
                simple_command!("baz", location!(13))
            )]]
        );
        assert_parse!(
            "foo |& bar > baz",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                simple_command!(
                    "bar",
                    location!(8),
                    redirect![write_to("baz", location!(12))]
                )
            )]]
        );

        assert_parse!(
            "foo|&\nbar",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                simple_command!("bar", location!(1, 2))
            )]]
        );

        assert_parse!(
            "foo |& bar |& baz",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                connecter_pipe_both!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
        assert_parse!(
            "foo |& bar |& baz &",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                connecter_pipe_both!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15), None, true)
                )
            )]]
        );
        assert_parse!(
            "foo |& bar > baz |& qux",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                connecter_pipe_both!(
                    simple_command!(
                        "bar",
                        location!(8),
                        redirect![write_to("baz", location!(12))]
                    ),
                    simple_command!("qux", location!(21))
                )
            )]]
        );

        assert_parse!(
            "foo |& bar | baz",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                connecter_pipe!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(14))
                )
            )]]
        );
        assert_parse!(
            "foo |& bar && baz",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                connecter_and!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
        assert_parse!(
            "foo |& bar || baz",
            ok![vec![connecter_pipe_both!(
                simple_command!("foo"),
                connecter_or!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
    }

    #[test]
    fn connecter_and() {
        assert_parse!(
            "foo && bar",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                simple_command!("bar", location!(8))
            )]]
        );

        assert_parse!("foo &&", err![Error::eof(location!(7))]);
        assert_parse!(
            "foo& && bar",
            err![Error::unexpected_token(&Token::and(location!(6)))]
        );

        assert_parse!(
            "foo && bar&",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                simple_command!("bar", location!(8), None, true)
            )]]
        );

        assert_parse!(
            "foo > bar && baz",
            ok![vec![connecter_and!(
                simple_command!("foo", location!(), redirect![write_to("bar", location!(5))]),
                simple_command!("baz", location!(14))
            )]]
        );
        assert_parse!(
            "foo && bar > baz",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                simple_command!(
                    "bar",
                    location!(8),
                    redirect![write_to("baz", location!(12))]
                )
            )]]
        );

        assert_parse!(
            "foo &&\nbar",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                simple_command!("bar", location!(1, 2))
            )]]
        );

        assert_parse!(
            "foo && bar && baz",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                connecter_and!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
        assert_parse!(
            "foo && bar && baz &",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                connecter_and!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15), None, true)
                )
            )]]
        );
        assert_parse!(
            "foo && bar > baz && qux",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                connecter_and!(
                    simple_command!(
                        "bar",
                        location!(8),
                        redirect![write_to("baz", location!(12))]
                    ),
                    simple_command!("qux", location!(21))
                )
            )]]
        );

        assert_parse!(
            "foo && bar | baz",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                connecter_pipe!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(14))
                )
            )]]
        );
        assert_parse!(
            "foo && bar |& baz",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                connecter_pipe_both!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
        assert_parse!(
            "foo && bar || baz",
            ok![vec![connecter_and!(
                simple_command!("foo"),
                connecter_or!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
    }

    #[test]
    fn connecter_or() {
        assert_parse!(
            "foo || bar",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                simple_command!("bar", location!(8))
            )]]
        );

        assert_parse!("foo ||", err![Error::eof(location!(7))]);
        assert_parse!(
            "foo& || bar",
            err![Error::unexpected_token(&Token::or(location!(6)))]
        );

        assert_parse!(
            "foo || bar&",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                simple_command!("bar", location!(8), None, true)
            )]]
        );

        assert_parse!(
            "foo > bar || baz",
            ok![vec![connecter_or!(
                simple_command!("foo", location!(), redirect![write_to("bar", location!(5))]),
                simple_command!("baz", location!(14))
            )]]
        );
        assert_parse!(
            "foo || bar > baz",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                simple_command!(
                    "bar",
                    location!(8),
                    redirect![write_to("baz", location!(12))]
                )
            )]]
        );

        assert_parse!(
            "foo ||\nbar",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                simple_command!("bar", location!(1, 2))
            )]]
        );

        assert_parse!(
            "foo || bar || baz",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                connecter_or!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
        assert_parse!(
            "foo || bar || baz &",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                connecter_or!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15), None, true)
                )
            )]]
        );
        assert_parse!(
            "foo || bar > baz || qux",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                connecter_or!(
                    simple_command!(
                        "bar",
                        location!(8),
                        redirect![write_to("baz", location!(12))]
                    ),
                    simple_command!("qux", location!(21))
                )
            )]]
        );

        assert_parse!(
            "foo || bar | baz",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                connecter_pipe!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(14))
                )
            )]]
        );
        assert_parse!(
            "foo || bar |& baz",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                connecter_pipe_both!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
        assert_parse!(
            "foo || bar && baz",
            ok![vec![connecter_or!(
                simple_command!("foo"),
                connecter_and!(
                    simple_command!("bar", location!(8)),
                    simple_command!("baz", location!(15))
                )
            )]]
        );
    }

    #[test]
    fn if_statement() {
        assert_parse!(
            "if foo; then bar; baz; fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![
                    simple_command!("bar", location!(14)),
                    simple_command!("baz", location!(19)),
                ]
            )]]
        );
        assert_parse!(
            "if foo; then bar; fi > baz &",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(14))],
                None,
                redirect![write_to("baz", location!(22))],
                true
            )]]
        );
        assert_parse!(
            "if foo; then bar; else baz; qux; fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(14))],
                Some(vec![
                    simple_command!("baz", location!(24)),
                    simple_command!("qux", location!(29))
                ])
            )]]
        );
        assert_parse!(
            "if foo; bar; baz; fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![
                    simple_command!("bar", location!(9)),
                    simple_command!("baz", location!(14)),
                ]
            )]]
        );
        assert_parse!(
            "if foo; bar; baz; end",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![
                    simple_command!("bar", location!(9)),
                    simple_command!("baz", location!(14)),
                ]
            )]]
        );
        assert_parse!(
            "if foo; then bar; baz; end",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![
                    simple_command!("bar", location!(14)),
                    simple_command!("baz", location!(19)),
                ]
            )]]
        );
        assert_parse!(
            "if foo; bar; else baz; end",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(9))],
                Some(vec![simple_command!("baz", location!(19))])
            )]]
        );
        assert_parse!(
            "if foo; then bar; elif baz; then qux; fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(14))],
                Some(vec![if_statement!(
                    simple_command!("baz", location!(24)),
                    vec![simple_command!("qux", location!(34))]
                )])
            )]]
        );
        assert_parse!(
            "if foo; then bar; elif baz; then qux; else quux; fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(14))],
                Some(vec![if_statement!(
                    simple_command!("baz", location!(24)),
                    vec![simple_command!("qux", location!(34))],
                    Some(vec![simple_command!("quux", location!(44))])
                )])
            )]]
        );
        assert_parse!(
            "if foo; then bar; elif baz; then qux; elif quux; then corge; else; grault; fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(14))],
                Some(vec![if_statement!(
                    simple_command!("baz", location!(24)),
                    vec![simple_command!("qux", location!(34))],
                    Some(vec![if_statement![
                        simple_command!("quux", location!(44)),
                        vec![simple_command!("corge", location!(55))],
                        Some(vec![simple_command!("grault", location!(68))])
                    ]])
                )])
            )]]
        );
        assert_parse!(
            "if foo; then bar; elsif baz; then qux; end",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(14))],
                Some(vec![if_statement!(
                    simple_command!("baz", location!(25)),
                    vec![simple_command!("qux", location!(35))]
                )])
            )]]
        );
        assert_parse!(
            "if foo; then bar; elsif baz; then qux; else quux; end",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(14))],
                Some(vec![if_statement!(
                    simple_command!("baz", location!(25)),
                    vec![simple_command!("qux", location!(35))],
                    Some(vec![simple_command!("quux", location!(45))])
                )])
            )]]
        );
        assert_parse!(
            "if foo; bar; elsif baz; qux; else quux; end",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(9))],
                Some(vec![if_statement!(
                    simple_command!("baz", location!(20)),
                    vec![simple_command!("qux", location!(25))],
                    Some(vec![simple_command!("quux", location!(35))])
                )])
            )]]
        );
        assert_parse!(
            "if foo | bar; then baz; fi",
            ok![vec![if_statement!(
                connecter_pipe!(
                    simple_command!("foo", location!(4)),
                    simple_command!("bar", location!(10))
                ),
                vec![simple_command!("baz", location!(20))]
            )]]
        );
        assert_parse!(
            "if if foo; then bar; fi; then baz; fi",
            ok![vec![if_statement!(
                if_statement!(
                    simple_command!("foo", location!(7)),
                    vec![simple_command!("bar", location!(17))]
                ),
                vec![simple_command!("baz", location!(31))]
            )]]
        );
        assert_parse!(
            "if foo; then if bar; then baz; fi fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![if_statement!(
                    simple_command!("bar", location!(17)),
                    vec![simple_command!("baz", location!(27))]
                )]
            )]]
        );
        assert_parse!(
            "if foo; then if bar; then baz; fi; fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![if_statement!(
                    simple_command!("bar", location!(17)),
                    vec![simple_command!("baz", location!(27))]
                )]
            )]]
        );
        assert_parse!(
            "if foo; then bar; elif if baz; then qux; fi; then quux; fi",
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(14))],
                Some(vec![if_statement!(
                    if_statement!(
                        simple_command!("baz", location!(27)),
                        vec![simple_command!("qux", location!(37))]
                    ),
                    vec![simple_command!("quux", location!(51))]
                )])
            )]]
        );

        assert_parse!(
            r#"
              if foo
              then
                bar
              else
                baz
              fi
            "#,
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(3, 3))],
                Some(vec![simple_command!("baz", location!(3, 5))])
            )]]
        );
        assert_parse!(
            r#"
              if foo
                bar
              else
                baz
              fi
            "#,
            ok![vec![if_statement!(
                simple_command!("foo", location!(4)),
                vec![simple_command!("bar", location!(3, 2))],
                Some(vec![simple_command!("baz", location!(3, 4))])
            )]]
        );

        assert_parse!(
            r#"
              if foo; then
                bar
              fi | if baz
                qux
              end
            "#,
            ok![vec![connecter_pipe!(
                if_statement!(
                    simple_command!("foo", location!(4)),
                    vec![simple_command!("bar", location!(3, 2))]
                ),
                if_statement!(
                    simple_command!("baz", location!(9, 3)),
                    vec![simple_command!("qux", location!(3, 4))]
                )
            )]]
        );
    }

    #[test]
    fn unless_statement() {
        assert_parse!(
            "unless foo; then bar; baz; end",
            ok![vec![unless_statement!(
                simple_command!("foo", location!(8)),
                vec![
                    simple_command!("bar", location!(18)),
                    simple_command!("baz", location!(23)),
                ]
            )]]
        );
        assert_parse!(
            "unless foo; then bar; baz; fi",
            err![Error::unexpected_token(&Token::keyword(
                "fi",
                location!(28)
            ))]
        );
        assert_parse!(
            "unless foo; then bar; end > baz &",
            ok![vec![unless_statement!(
                simple_command!("foo", location!(8)),
                vec![simple_command!("bar", location!(18))],
                None,
                redirect![write_to("baz", location!(27))],
                true
            )]]
        );
        assert_parse!(
            "unless foo; then bar; else baz; qux; fi",
            ok![vec![unless_statement!(
                simple_command!("foo", location!(8)),
                vec![simple_command!("bar", location!(18))],
                Some(vec![
                    simple_command!("baz", location!(28)),
                    simple_command!("qux", location!(33))
                ])
            )]]
        );
        assert_parse!(
            "unless foo; bar; baz; end",
            ok![vec![unless_statement!(
                simple_command!("foo", location!(8)),
                vec![
                    simple_command!("bar", location!(13)),
                    simple_command!("baz", location!(18)),
                ]
            )]]
        );
        assert_parse!(
            "unless foo; then bar; elsif baz; then qux; end",
            err![Error::unexpected_token(&Token::keyword(
                "elsif",
                location!(23)
            ))]
        );
        assert_parse!(
            "unless foo; then bar; elif baz; then qux; end",
            err![Error::unexpected_token(&Token::keyword(
                "elif",
                location!(23)
            ))]
        );
        assert_parse!(
            "unless foo | bar; then baz; end",
            ok![vec![unless_statement!(
                connecter_pipe!(
                    simple_command!("foo", location!(8)),
                    simple_command!("bar", location!(14))
                ),
                vec![simple_command!("baz", location!(24))]
            )]]
        );
        assert_parse!(
            "unless if foo; then bar; fi; then baz; end",
            ok![vec![unless_statement!(
                if_statement!(
                    simple_command!("foo", location!(11)),
                    vec![simple_command!("bar", location!(21))]
                ),
                vec![simple_command!("baz", location!(35))]
            )]]
        );
        assert_parse!(
            "unless foo; then if bar; then baz; fi end",
            ok![vec![unless_statement!(
                simple_command!("foo", location!(8)),
                vec![if_statement!(
                    simple_command!("bar", location!(21)),
                    vec![simple_command!("baz", location!(31))]
                )]
            )]]
        );
        assert_parse!(
            "unless foo; then if bar; then baz; fi; end",
            ok![vec![unless_statement!(
                simple_command!("foo", location!(8)),
                vec![if_statement!(
                    simple_command!("bar", location!(21)),
                    vec![simple_command!("baz", location!(31))]
                )]
            )]]
        );

        assert_parse!(
            r#"
              unless foo
              then
                bar
              else
                baz
              fi
            "#,
            ok![vec![unless_statement!(
                simple_command!("foo", location!(8)),
                vec![simple_command!("bar", location!(3, 3))],
                Some(vec![simple_command!("baz", location!(3, 5))])
            )]]
        );
        assert_parse!(
            r#"
              unless foo
                bar
              else
                baz
              fi
            "#,
            ok![vec![unless_statement!(
                simple_command!("foo", location!(8)),
                vec![simple_command!("bar", location!(3, 2))],
                Some(vec![simple_command!("baz", location!(3, 4))])
            )]]
        );

        assert_parse!(
            r#"
              unless foo; then
                bar
              end | if baz
                qux
              end
            "#,
            ok![vec![connecter_pipe!(
                unless_statement!(
                    simple_command!("foo", location!(8)),
                    vec![simple_command!("bar", location!(3, 2))]
                ),
                if_statement!(
                    simple_command!("baz", location!(10, 3)),
                    vec![simple_command!("qux", location!(3, 4))]
                )
            )]]
        );
    }

    #[test]
    fn while_statement() {
        assert_parse!(
            "while foo; do bar; done",
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(15))]
            )]]
        );
        assert_parse!(
            "while foo; do bar; end",
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(15))]
            )]]
        );
        assert_parse!(
            "while foo; bar; done",
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(12))]
            )]]
        );
        assert_parse!(
            "while foo; bar; end",
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(12))]
            )]]
        );

        assert_parse!(
            "while foo; do bar; done > baz",
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(15))],
                redirect![write_to("baz", location!(25))]
            )]]
        );
        assert_parse!(
            "while foo; do bar; done &",
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(15))],
                None,
                true
            )]]
        );
        assert_parse!(
            "while foo; do bar; done | while baz; qux; end",
            ok![vec![connecter_pipe!(
                while_statement!(
                    simple_command!("foo", location!(7)),
                    vec![simple_command!("bar", location!(15))]
                ),
                while_statement!(
                    simple_command!("baz", location!(33)),
                    vec![simple_command!("qux", location!(38))]
                )
            )]]
        );
        assert_parse!(
            "while if foo; then bar; fi; do baz; done",
            ok![vec![while_statement!(
                if_statement!(
                    simple_command!("foo", location!(10)),
                    vec![simple_command!("bar", location!(20))]
                ),
                vec![simple_command!("baz", location!(32))]
            )]]
        );
        assert_parse!(
            "while foo; do while bar; baz; end; done",
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![while_statement!(
                    simple_command!("bar", location!(21)),
                    vec![simple_command!("baz", location!(26))]
                )]
            )]]
        );

        assert_parse!(
            r#"
              while foo
              do
                bar
              done
            "#,
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(3, 3))]
            )]]
        );
        assert_parse!(
            r#"
              while foo
                bar
              done
            "#,
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(3, 2))]
            )]]
        );
        assert_parse!(
            r#"
              while foo
                bar
              end
            "#,
            ok![vec![while_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(3, 2))]
            )]]
        );
    }

    #[test]
    fn until_statement() {
        assert_parse!(
            "until foo; do bar; done",
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(15))]
            )]]
        );
        assert_parse!(
            "until foo; do bar; end",
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(15))]
            )]]
        );
        assert_parse!(
            "until foo; bar; done",
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(12))]
            )]]
        );
        assert_parse!(
            "until foo; bar; end",
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(12))]
            )]]
        );

        assert_parse!(
            "until foo; do bar; done > baz",
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(15))],
                redirect![write_to("baz", location!(25))]
            )]]
        );
        assert_parse!(
            "until foo; do bar; done &",
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(15))],
                None,
                true
            )]]
        );
        assert_parse!(
            "until foo; do bar; done | until baz; qux; end",
            ok![vec![connecter_pipe!(
                until_statement!(
                    simple_command!("foo", location!(7)),
                    vec![simple_command!("bar", location!(15))]
                ),
                until_statement!(
                    simple_command!("baz", location!(33)),
                    vec![simple_command!("qux", location!(38))]
                )
            )]]
        );
        assert_parse!(
            "until if foo; then bar; fi; do baz; done",
            ok![vec![until_statement!(
                if_statement!(
                    simple_command!("foo", location!(10)),
                    vec![simple_command!("bar", location!(20))]
                ),
                vec![simple_command!("baz", location!(32))]
            )]]
        );
        assert_parse!(
            "until foo; do until bar; baz; end; done",
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![until_statement!(
                    simple_command!("bar", location!(21)),
                    vec![simple_command!("baz", location!(26))]
                )]
            )]]
        );

        assert_parse!(
            r#"
              until foo
              do
                bar
              done
            "#,
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(3, 3))]
            )]]
        );
        assert_parse!(
            r#"
              until foo
                bar
              done
            "#,
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(3, 2))]
            )]]
        );
        assert_parse!(
            r#"
              until foo
                bar
              end
            "#,
            ok![vec![until_statement!(
                simple_command!("foo", location!(7)),
                vec![simple_command!("bar", location!(3, 2))]
            )]]
        );
    }

    #[test]
    fn for_statement() {
        assert_parse!(
            "for foo in bar baz; do qux; done",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                Some(vec![
                    vec![Word::normal("bar", location!(12))],
                    vec![Word::normal("baz", location!(16))],
                ]),
                vec![simple_command!("qux", location!(24))]
            )]]
        );
        assert_parse!(
            "for foo in bar baz; do qux; end",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                Some(vec![
                    vec![Word::normal("bar", location!(12))],
                    vec![Word::normal("baz", location!(16))],
                ]),
                vec![simple_command!("qux", location!(24))]
            )]]
        );
        assert_parse!(
            "for foo in bar baz; qux; end",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                Some(vec![
                    vec![Word::normal("bar", location!(12))],
                    vec![Word::normal("baz", location!(16))],
                ]),
                vec![simple_command!("qux", location!(21))]
            )]]
        );
        assert_parse!(
            "for foo; do bar; done",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                vec![simple_command!("bar", location!(13))]
            )]]
        );
        assert_parse!(
            "for foo; do bar; end",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                vec![simple_command!("bar", location!(13))]
            )]]
        );
        assert_parse!(
            "for foo; bar; done",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                vec![simple_command!("bar", location!(10))]
            )]]
        );
        assert_parse!(
            "for foo; bar; end",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                vec![simple_command!("bar", location!(10))]
            )]]
        );

        assert_parse!(
            "for foo in bar baz; do qux; done > quux",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                Some(vec![
                    vec![Word::normal("bar", location!(12))],
                    vec![Word::normal("baz", location!(16))],
                ]),
                vec![simple_command!("qux", location!(24))],
                redirect![write_to("quux", location!(34))]
            )]]
        );
        assert_parse!(
            "for foo in bar baz; do qux; done &",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                Some(vec![
                    vec![Word::normal("bar", location!(12))],
                    vec![Word::normal("baz", location!(16))],
                ]),
                vec![simple_command!("qux", location!(24))],
                None,
                true
            )]]
        );

        assert_parse!(
            "for foo; do if bar; then baz; fi; done",
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                vec![if_statement!(
                    simple_command!("bar", location!(16)),
                    vec![simple_command!("baz", location!(26))]
                )]
            )]]
        );

        assert_parse!(
            r#"
              for foo in bar baz
              do
                qux
              done
            "#,
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                Some(vec![
                    vec![Word::normal("bar", location!(12))],
                    vec![Word::normal("baz", location!(16))],
                ]),
                vec![simple_command!("qux", location!(3, 3))]
            )]]
        );
        assert_parse!(
            r#"
              for foo in bar baz
                qux
              done
            "#,
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                Some(vec![
                    vec![Word::normal("bar", location!(12))],
                    vec![Word::normal("baz", location!(16))],
                ]),
                vec![simple_command!("qux", location!(3, 2))]
            )]]
        );
        assert_parse!(
            r#"
              for foo in bar baz
                qux
              end
            "#,
            ok![vec![for_statement!(
                vec![Word::normal("foo", location!(5))],
                Some(vec![
                    vec![Word::normal("bar", location!(12))],
                    vec![Word::normal("baz", location!(16))],
                ]),
                vec![simple_command!("qux", location!(3, 2))]
            )]]
        );
    }
}
