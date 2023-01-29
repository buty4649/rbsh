extern crate rbsh_parser;

#[macro_use]
mod common;

mod test_command {
    use indoc::indoc;
    use rbsh_parser::ast::*;
    use rbsh_parser::parse;
    use pretty_assertions::assert_eq;

    #[test]
    fn comment() {
        assert_parse!("# comment" => Ok(vec![]));
        assert_parse!("foo bar baz # comment" => Ok(vec![command!(
            name: vec![bare!(foo)],
            args: vec![vec![bare!(bar)], vec![bare!(baz)]]
        )]));
    }

    #[test]
    fn bare_word() {
        assert_parse!("foo bar baz" => Ok(vec![command!(
            name: vec![bare!(foo)],
            args: vec![vec![bare!(bar)], vec![bare!(baz)]]
        )]));
        assert_parse!(indoc!(r#"
                foo bar\
                baz
            "#) => Ok(vec![command!(
            name: vec![bare!(foo)],
            args: vec![vec![bare!(barbaz)]]
        )]));

        let keywords = [
            ' ', '\t', '\'', '\'', '`', '<', '>', '#', '(', ')', '$', ';', '|', '&',
        ];

        for keyword in keywords {
            let input = format!("foo\\{}bar", keyword);
            let expect = format!("foo{}bar", keyword);
            assert_parse!(&input => Ok(vec![command!(
                name: vec![bare!(@ &expect)]
            )]));
        }
    }

    #[test]
    fn quote_word() {
        assert_parse!(r#""foo""# => Ok(vec![command!(
            name: vec![quote![bare!(foo)]]
        )]));
        assert_parse!(r#""\\""# => Ok(vec![command!(
            name: vec![quote![bare!(@ r#"\"#)]]
        )]));
        assert_parse!(r#""\`""# => Ok(vec![command!(
            name: vec![quote![bare!(@ "`")]]
        )]));
        assert_parse!(r#""\$""# => Ok(vec![command!(
            name: vec![quote![bare!($)]]
        )]));
        assert_parse!(indoc!(r#"
                "foo
                bar"
            "#) => Ok(vec![command!(
            name: vec![quote![bare!(@ "foo\nbar")]]
        )]));
        assert_parse!(indoc!(r#"
                "foo\
                bar"
            "#) => Ok(vec![command!(
            name: vec![quote![bare!(@ "foobar")]]
        )]));
        assert_parse!(r#""foo\"bar\"baz""# => Ok(vec![command!(
            name: vec![quote!(bare!(r#"foo"bar"baz"#))]
        )]));

        assert_parse!("'foo'" => Ok(vec![command!(
            name: vec![quote![bare!(foo)]]
        )]));
        assert_parse!(r#"'foo\'"# => Ok(vec![command!(
            name: vec![quote![bare!(@ "foo\\")]]
        )]));
        assert_parse!(indoc!("
                'foo
                bar'
            ") => Ok(vec![command!(
            name: vec![quote![bare!(@ "foo\nbar")]]
        )]));
        assert_parse!(indoc!(r#"
                'foo\
                bar'
            "#) => Ok(vec![command!(
            name: vec![quote![bare!(@ "foo\\\nbar")]]
        )]));
        assert_parse!(r#"'foo\'bar\'baz'"# => Ok(vec![command!(
            name: vec![quote!(bare!("foo'bar'baz"))]
        )]));

        assert_parse!(r#"foo"bar"'baz'"# => Ok(vec![command!(
            name: vec![bare!(foo), quote![bare!(bar)], quote![bare!(baz)]]
        )]));
        assert_parse!(r#""foo'bar""# => Ok(vec![command!(
            name: vec![quote!(bare!(r#""foo'bar""#))]
        )]));
        assert_parse!(r#"'foo"bar'"# => Ok(vec![command!(
            name: vec![quote!(bare!(r#"foo"bar"#))]
        )]));

        assert_parse_error!(r#""foo"#);
        assert_parse_error!("'foo'bar'");

        assert_parse!(r#"$'\afoo\e'"# => Ok(vec![command!(
            name: vec![quote![bare!(@"\u{7}foo\u{1b}")]]
        )]));

        assert_parse_error!(r#"$"foobar""#);
    }

    #[test]
    fn command_substitute() {
        assert_parse!("`foo`" => Ok(vec![command!(
            name: vec![command_sub!(command!(name: vec![bare!(foo)]))]
        )]));
        assert_parse!(r#"`foo \`bar\``"# => Ok(vec![command!(
            name: vec![command_sub!(command!(
                name: vec![bare!(foo)],
                args: vec![vec![command_sub!(command!(
                    name: vec![bare!(bar)]
                ))]]
            ))]
        )]));
        assert_parse!(indoc!("
                `foo\
                bar`
            ") => Ok(vec![command!(
            name: vec![command_sub!(command!(name: vec![bare!(foobar)]))]
        )]));
        assert_parse!("`foo `bar``" => Ok(vec![command!(
            name: vec![
                command_sub!(command!(name: vec![bare!(foo)])),
                bare!(bar),
                command_sub!(),
            ]
        )]));
        assert_parse!("`$(foo)`" => Ok(vec![command!(
            name: vec![command_sub!(command!(
                name: vec![command_sub!(command!(
                    name: vec![bare!(foo)]
                ))
                ]
            ))]
        )]));
        assert_parse_error!("`$(`foo`)`");

        assert_parse!("$(foo)" => Ok(vec![command!(
            name: vec![command_sub!(command!(name: vec![bare!(foo)]))]
        )]));
        assert_parse!(r#"$(foo\$\(bar\))"# => Ok(vec![command!(
            name: vec![command_sub!(command!(name: vec![bare!("foo$(bar)")]))]
        )]));
        assert_parse!(indoc!("
                $(foo\
                bar)
            ") => Ok(vec![command!(
                name: vec![command_sub!(command!(name: vec![bare!(foobar)]))]
        )]));
        assert_parse!("$($(foo))" => Ok(vec![command!(
            name: vec![command_sub!(command!(
                name: vec![command_sub!(command!(
                    name: vec![bare!(foo)]
                ))]
            ))]
        )]));
        assert_parse!("$(`$(foo)`)" => Ok(vec![command!(
            name: vec![command_sub!(command!(
                name: vec![command_sub!(command!(
                    name: vec![command_sub!(command!(name: vec![bare!(foo)]))]
                ))]
            ))]
        )]));
        assert_parse!(r#""`$("foo")`""# => Ok(vec![command!(
            name: vec![quote!(
                command_sub!(command!(
                    name: vec![command_sub!(command!(
                        name: vec![quote!(bare!(foo))]
                    ))]
                ))
            )]
        )]));
        assert_parse!(r#""$(`"foo"`)""# => Ok(vec![command!(
            name: vec![quote!(
                command_sub!(command!(
                    name: vec![command_sub!(command!(
                        name: vec![quote!(bare!(foo))]
                    ))]
                ))
            )]
        )]));
    }

    #[test]
    fn parameter_substitute() {
        assert_parse!("$foo" => Ok(vec![command!(
            name: vec![param_sub!(foo)]
        )]));
        assert_parse!("$foo_bar" => Ok(vec![command!(
            name: vec![param_sub!(foo_bar)]
        )]));
        assert_parse!(indoc!(r#"
                $\
                \
                foo
            "#) => Ok(vec![command!(
            name: vec![param_sub!(foo)]
        )]));
        assert_parse!(indoc!(r#"
                $foo\
                bar
            "#) => Ok(vec![command!(
            name: vec![param_sub!(foobar)]
        )]));
        assert_parse!("$foo#comment" => Ok(vec![command!(
            name: vec![param_sub!(foo)]
        )]));
        assert_parse!("$" => Ok(vec![command!(
            name: vec![bare!($)]
        )]));

        assert_parse!(r#"$foo"bar""# => Ok(vec![command!(
            name: vec![param_sub!(foo), quote!(bare!(bar))]
        )]));
        assert_parse!("${foo}" => Ok(vec![command!(
            name: vec![param_sub!(foo)]
        )]));
        assert_parse_error!("${foo");

        assert_parse!("$foo${bar}" => Ok(vec![command!(
            name: vec![param_sub!(foo), param_sub!(bar)]
        )]));
        assert_parse!(r#""$foo""${bar}""# => Ok(vec![command!(
            name: vec![quote!(param_sub!(foo)), quote!(param_sub!(bar))]
        )]));

        assert_parse_error!(r#""${foo""#);
    }

    #[test]
    fn special_variable() {
        let marks = ["*", "@", "#", "?", "-", "$", "!", "_", "0", "10"];

        for mark in marks {
            let input = format!("${}", mark);
            assert_parse!(&input => Ok(vec![command!(name: vec![param_sub!(@ mark)])]));
        }

        /*
        for mark in marks {
            let input = format!("${{{}}}", mark);
            assert_parse!(&input => Ok(vec![command!(name: vec![param_sub!(@ mark)])]));
        }
        */

        assert_parse!("$*foo" => Ok(vec![command!(
            name: vec![
                param_sub!(*),
                bare!(foo)
            ]
        )]));
        assert_parse!("$0foo" => Ok(vec![command!(
            name: vec![
                param_sub!(0),
                bare!(foo)
            ]
        )]));
        assert_parse!("$10foo" => Ok(vec![command!(
            name: vec![
                param_sub!(10),
                bare!(foo)
            ]
        )]));
        assert_parse!("$09foo" => Ok(vec![command!(
            name: vec![
                param_sub!(0),
                bare!(9foo)
            ]
        )]));
        assert_parse!(indoc!(r#"
                $1\
                0
            "#) => Ok(vec![command!(name: vec![param_sub!(10)])]));
    }

    #[test]
    fn variable_assingment() {
        assert_parse!("FOO=" => Ok(vec![variable_assignment!(parameter!(FOO))]));
        assert_parse!("_FOO=" => Ok(vec![variable_assignment!(parameter!(_FOO))]));
        assert_parse!("FOO=bar HOGE=hige" => Ok(vec![variable_assignment!(
            parameter!(FOO, vec![bare!(bar)]),
            parameter!(HOGE, vec![bare!(hige)]),
        )]));
        assert_parse!(r#"FOO="bar"'baz'`hoge`$(hige)"# => Ok(vec![variable_assignment!(
            parameter!(
                FOO,
                vec![
                    quote!(bare!(bar)),
                    quote!(bare!(baz)),
                    command_sub!(command!(name: vec![bare!(hoge)])),
                    command_sub!(command!(name: vec![bare!(hige)])),
                ]
            )
        )]));

        assert_parse!("FOO=bar HOGE=hige piyo" => Ok(vec![command!(
            name: vec![bare!(piyo)],
            parameter: vec![
                parameter!(FOO, vec![bare!(bar)]),
                parameter!(HOGE, vec![bare!(hige)]),
            ]
        )]));
        assert_parse!("foo HOGE=hige" => Ok(vec![command!(
            name: vec![bare!(foo)],
            args: vec![
                vec![bare!(@ "HOGE=hige")]
            ]
        )]));
    }

}
