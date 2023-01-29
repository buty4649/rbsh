extern crate rbsh_parser;

#[macro_use]
mod common;

mod test_redirect {
    use pretty_assertions::assert_eq;
    use rbsh_parser::ast::*;
    use rbsh_parser::parse;

    #[test]
    fn read_from() {
        assert_parse!("foo <bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_from!(0, vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo < bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_from!(0, vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo 3<bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_from!(3, vec![bare!(bar)])
            ]
        )]));
        assert_parse!("<bar foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_from!(0, vec![bare!(bar)])
            ]
        )]));
        assert_parse_error!("foo <");
    }

    #[test]
    fn write_to() {
        assert_parse!("foo >bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_to!(1, vec![bare!(bar)], false)
            ]
        )]));
        assert_parse!("foo > bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_to!(1, vec![bare!(bar)], false)
            ]
        )]));
        assert_parse!("foo 2>bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_to!(2, vec![bare!(bar)], false)
            ]
        )]));
        assert_parse!(">bar foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_to!(1, vec![bare!(bar)], false)
            ]
        )]));
        assert_parse_error!("foo >");

        // force
        assert_parse!("foo >|bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_to!(1, vec![bare!(bar)], true)
            ]
        )]));
        assert_parse!("foo >| bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_to!(1, vec![bare!(bar)], true)
            ]
        )]));
        assert_parse!("foo 2>|bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_to!(2, vec![bare!(bar)], true)
            ]
        )]));
        assert_parse!(">|bar foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_to!(1, vec![bare!(bar)], true)
            ]
        )]));
        assert_parse_error!("foo >|");
    }

    #[test]
    fn write_both() {
        // write both
        assert_parse!("foo &>bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_both!(vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo &> bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_both!(vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo >&bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_both!(vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo >& bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_both!(vec![bare!(bar)])
            ]
        )]));
    }

    #[test]
    fn read_copy() {
        assert_parse!("foo <&2" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_copy!(0, 2, false)
            ]
        )]));
        assert_parse!("foo <& 2" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_copy!(0, 2, false)
            ]
        )]));
        assert_parse!("foo <&2-" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_copy!(0, 2, true)
            ]
        )]));
        assert_parse!("foo <& 2-" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_copy!(0, 2, true)
            ]
        )]));
        assert_parse!("<&2 foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_copy!(0, 2, false)
            ]
        )]));
        assert_parse_error!("foo >&");
    }

    #[test]
    fn write_copy() {
        assert_parse!("foo >&2" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_copy!(1, 2, false)
            ]
        )]));
        assert_parse_error!("foo >&");
        assert_parse!("foo >& 2" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_copy!(1, 2, false)
            ]
        )]));
        assert_parse!("foo >&2-" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_copy!(1, 2, true)
            ]
        )]));
        assert_parse!("foo >& 2-" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_copy!(1, 2, true)
            ]
        )]));
        assert_parse!("foo >&2 -" => Ok(vec![command!(
            name: vec![bare!(foo)],
            args: vec![vec![bare!(-)]],
            redirect: vec![
                redirect_write_copy!(1, 2, false)
            ]
        )]));
        assert_parse!(">&2 foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_copy!(1, 2, false)
            ]
        )]));
        assert_parse_error!("foo <&");
    }

    #[test]
    fn read_close() {
        assert_parse!("foo <&-" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_close!(0)
            ]
        )]));
        assert_parse!("<&- foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_close!(0)
            ]
        )]));
        assert_parse_error!("foo <& -");
    }

    #[test]
    fn write_close() {
        assert_parse!("foo >&-" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_close!(1)
            ]
        )]));
        assert_parse!(">&- foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_close!(1)
            ]
        )]));
        assert_parse_error!("foo >& -");
    }

    #[test]
    fn append() {
        assert_parse!("foo >>bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_append_to!(1, vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo >> bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_append_to!(1, vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo 2>>bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_append_to!(2, vec![bare!(bar)])
            ]
        )]));
        assert_parse!(">>bar foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_append_to!(1, vec![bare!(bar)])
            ]
        )]));
        assert_parse_error!("foo >>");
    }

    #[test]
    fn append_both() {
        assert_parse!("foo &>>bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_append_both!(vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo &>> bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_append_both!(vec![bare!(bar)])
            ]
        )]));
        assert_parse!("&>>bar foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_append_both!(vec![bare!(bar)])
            ]
        )]));
        assert_parse_error!("foo &>>");
    }

    #[test]
    fn read_write() {
        assert_parse!("foo <>bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_write!(0, vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo <> bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_write!(0, vec![bare!(bar)])
            ]
        )]));
        assert_parse!("foo 2<>bar" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_write!(2, vec![bare!(bar)])
            ]
        )]));
        assert_parse!("<>bar foo" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_write!(0, vec![bare!(bar)])
            ]
        )]));
        assert_parse_error!("foo <>");
    }

    #[test]
    fn here_string() {
        assert_parse!(r#"foo <<<bar"baz""# => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_here_string!(0, vec![
                    bare!(bar),
                    quote!(bare!(baz))
                ])
            ]
        )]));
        assert_parse!(r#"<<<bar"baz" foo"# => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_here_string!(0, vec![
                    bare!(bar),
                    quote!(bare!(baz))
                ])
            ]
        )]));
        assert_parse_error!("foo <<<");
    }

    #[test]
    fn complex() {
        assert_parse!("foo < bar > baz" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_from!(0, vec![bare!(bar)]),
                redirect_write_to!(1, vec![bare!(baz)], false),
            ]
        )]));
        assert_parse!("< bar foo > baz" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_read_from!(0, vec![bare!(bar)]),
                redirect_write_to!(1, vec![bare!(baz)], false),
            ]
        )]));
        assert_parse!("foo >& bar &> baz >& 2-" => Ok(vec![command!(
            name: vec![bare!(foo)],
            redirect: vec![
                redirect_write_both!(vec![bare!(bar)]),
                redirect_write_both!(vec![bare!(baz)]),
                redirect_write_copy!(1, 2, true),
            ]
        )]));
    }
}
