use indoc::indoc;
use rbsh_parser::{location, Error, Lexer, Location, Token, WordKind};

macro_rules! assert_lex {
    ($s:expr, $($token:expr),+) => {{
        assert_eq!(
            Lexer::new($s, 0).iter().collect::<Vec<_>>(),
            vec![$($token,)+]
        );
    }};
}

#[test]
fn it_simple_command() {
    assert_lex!(
        "echo foo bar",
        Ok(Token::word("echo", WordKind::Normal, location!())),
        Ok(Token::space(location!(5))),
        Ok(Token::word("foo", WordKind::Normal, location!(6))),
        Ok(Token::space(location!(9))),
        Ok(Token::word("bar", WordKind::Normal, location!(10)))
    );

    assert_lex!(
        "echo 'foo bar'",
        Ok(Token::word("echo", WordKind::Normal, location!())),
        Ok(Token::space(location!(5))),
        Ok(Token::word("foo bar", WordKind::Quote, location!(6)))
    );

    assert_lex!(
        "echo \"foo bar\"",
        Ok(Token::word("echo", WordKind::Normal, location!())),
        Ok(Token::space(location!(5))),
        Ok(Token::word("foo bar", WordKind::Normal, location!(7)))
    );

    assert_lex!(
        "echo if",
        Ok(Token::word("echo", WordKind::Normal, location!())),
        Ok(Token::space(location!(5))),
        Ok(Token::word("if", WordKind::Normal, location!(6)))
    );

    assert_lex!(
        "echo < foo",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::read_from(location!(6, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1)))
    );

    assert_lex!(
        "echo 3< foo",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::number("3", location!(6, 1))),
        Ok(Token::read_from(location!(7, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(9, 1)))
    );

    assert_lex!(
        "echo > foo",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::write_to(location!(6, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1)))
    );

    assert_lex!(
        "echo >3 foo",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::write_to(location!(6, 1))),
        Ok(Token::word("3", WordKind::Normal, location!(7, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(9, 1)))
    );

    assert_lex!(
        "echo >| foo",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::force_write_to(location!(6, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(9, 1)))
    );

    assert_lex!(
        "echo >|3 foo",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::force_write_to(location!(6, 1))),
        Ok(Token::word("3", WordKind::Normal, location!(8, 1))),
        Ok(Token::space(location!(9, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(10, 1)))
    );

    assert_lex!(
        "> foo echo bar 2>&1 baz",
        Ok(Token::write_to(location!())),
        Ok(Token::space(location!(2))),
        Ok(Token::word("foo", WordKind::Normal, location!(3))),
        Ok(Token::space(location!(6))),
        Ok(Token::word("echo", WordKind::Normal, location!(7))),
        Ok(Token::space(location!(11))),
        Ok(Token::word("bar", WordKind::Normal, location!(12))),
        Ok(Token::space(location!(15))),
        Ok(Token::number("2", location!(16))),
        Ok(Token::write_copy(location!(17))),
        Ok(Token::number("1", location!(19))),
        Ok(Token::space(location!(20))),
        Ok(Token::word("baz", WordKind::Normal, location!(21)))
    );

    assert_lex!(
        "echo foo | echo bar",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(6, 1))),
        Ok(Token::space(location!(9, 1))),
        Ok(Token::pipe(location!(10, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::word("echo", WordKind::Normal, location!(12, 1))),
        Ok(Token::space(location!(16, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(17, 1)))
    );

    assert_lex!(
        "echo foo |& echo bar",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(6, 1))),
        Ok(Token::space(location!(9, 1))),
        Ok(Token::pipe_both(location!(10, 1))),
        Ok(Token::space(location!(12, 1))),
        Ok(Token::word("echo", WordKind::Normal, location!(13, 1))),
        Ok(Token::space(location!(17, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(18, 1)))
    );

    assert_lex!(
        "echo foo; echo bar",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(6, 1))),
        Ok(Token::termination(location!(9, 1))),
        Ok(Token::space(location!(10, 1))),
        Ok(Token::word("echo", WordKind::Normal, location!(11, 1))),
        Ok(Token::space(location!(15, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(16, 1)))
    );

    assert_lex!(
        "echo foo; echo bar | echo baz",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(6, 1))),
        Ok(Token::termination(location!(9, 1))),
        Ok(Token::space(location!(10, 1))),
        Ok(Token::word("echo", WordKind::Normal, location!(11, 1))),
        Ok(Token::space(location!(15, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(16, 1))),
        Ok(Token::space(location!(19, 1))),
        Ok(Token::pipe(location!(20, 1))),
        Ok(Token::space(location!(21, 1))),
        Ok(Token::word("echo", WordKind::Normal, location!(22, 1))),
        Ok(Token::space(location!(26, 1))),
        Ok(Token::word("baz", WordKind::Normal, location!(27, 1)))
    );

    assert_lex!(
        "echo > foo | echo > bar; echo baz",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Ok(Token::write_to(location!(6, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::pipe(location!(12, 1))),
        Ok(Token::space(location!(13, 1))),
        Ok(Token::word("echo", WordKind::Normal, location!(14, 1))),
        Ok(Token::space(location!(18, 1))),
        Ok(Token::write_to(location!(19, 1))),
        Ok(Token::space(location!(20, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(21, 1))),
        Ok(Token::termination(location!(24, 1))),
        Ok(Token::space(location!(25, 1))),
        Ok(Token::word("echo", WordKind::Normal, location!(26, 1))),
        Ok(Token::space(location!(30, 1))),
        Ok(Token::word("baz", WordKind::Normal, location!(31, 1)))
    );

    assert_lex!(
        "echo \"foo",
        Ok(Token::word("echo", WordKind::Normal, location!(1, 1))),
        Ok(Token::space(location!(5, 1))),
        Err(Error::unterminated_string(location!(7)))
    );
}

#[test]
fn it_if_statement() {
    assert_lex!(
        "if foo; then bar; fi",
        Ok(Token::keyword("if", location!())),
        Ok(Token::space(location!(3))),
        Ok(Token::word("foo", WordKind::Normal, location!(4))),
        Ok(Token::termination(location!(7))),
        Ok(Token::space(location!(8))),
        Ok(Token::keyword("then", location!(9))),
        Ok(Token::space(location!(13))),
        Ok(Token::word("bar", WordKind::Normal, location!(14))),
        Ok(Token::termination(location!(17))),
        Ok(Token::space(location!(18))),
        Ok(Token::keyword("fi", location!(19)))
    );

    assert_lex!(
        indoc! {r#"
          if foo
            bar
          elsif baz
            quz
          elsif quux
            corge
          else
            grault
          end
        "#},
        Ok(Token::keyword("if", location!())),
        Ok(Token::space(location!(3))),
        Ok(Token::word("foo", WordKind::Normal, location!(4))),
        Ok(Token::newline(location!(7))),
        Ok(Token::space(location!(1, 2))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 2))),
        Ok(Token::newline(location!(6, 2))),
        Ok(Token::keyword("elsif", location!(1, 3))),
        Ok(Token::space(location!(6, 3))),
        Ok(Token::word("baz", WordKind::Normal, location!(7, 3))),
        Ok(Token::newline(location!(10, 3))),
        Ok(Token::space(location!(1, 4))),
        Ok(Token::word("quz", WordKind::Normal, location!(3, 4))),
        Ok(Token::newline(location!(6, 4))),
        Ok(Token::keyword("elsif", location!(1, 5))),
        Ok(Token::space(location!(6, 5))),
        Ok(Token::word("quux", WordKind::Normal, location!(7, 5))),
        Ok(Token::newline(location!(11, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("corge", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(8, 6))),
        Ok(Token::keyword("else", location!(1, 7))),
        Ok(Token::newline(location!(5, 7))),
        Ok(Token::space(location!(1, 8))),
        Ok(Token::word("grault", WordKind::Normal, location!(3, 8))),
        Ok(Token::newline(location!(9, 8))),
        Ok(Token::keyword("end", location!(1, 9))),
        Ok(Token::newline(location!(4, 9)))
    );

    assert_lex!(
        indoc! {r#"
          if foo
          then if bar
              baz
            fi
          else
            qux
          fi
        "#},
        Ok(Token::keyword("if", location!(1, 1))),
        Ok(Token::space(location!(3, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(4, 1))),
        Ok(Token::newline(location!(7, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::space(location!(5, 2))),
        Ok(Token::keyword("if", location!(6, 2))),
        Ok(Token::space(location!(8, 2))),
        Ok(Token::word("bar", WordKind::Normal, location!(9, 2))),
        Ok(Token::newline(location!(12, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("baz", WordKind::Normal, location!(5, 3))),
        Ok(Token::newline(location!(8, 3))),
        Ok(Token::space(location!(1, 4))),
        Ok(Token::keyword("fi", location!(3, 4))),
        Ok(Token::newline(location!(5, 4))),
        Ok(Token::keyword("else", location!(1, 5))),
        Ok(Token::newline(location!(5, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("fi", location!(1, 7))),
        Ok(Token::newline(location!(3, 7)))
    );

    assert_lex!(
        indoc! {r#"
          if if foo
          then
            bar
          fi
          then
            baz
          else
            qux
          fi
        "#},
        Ok(Token::keyword("if", location!(1, 1))),
        Ok(Token::space(location!(3, 1))),
        Ok(Token::keyword("if", location!(4, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("fi", location!(1, 4))),
        Ok(Token::newline(location!(3, 4))),
        Ok(Token::keyword("then", location!(1, 5))),
        Ok(Token::newline(location!(5, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("baz", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("else", location!(1, 7))),
        Ok(Token::newline(location!(5, 7))),
        Ok(Token::space(location!(1, 8))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 8))),
        Ok(Token::newline(location!(6, 8))),
        Ok(Token::keyword("fi", location!(1, 9))),
        Ok(Token::newline(location!(3, 9)))
    );

    assert_lex!(
        indoc! {r#"
          if foo
          then
            bar
          fi > baz
        "#},
        Ok(Token::keyword("if", location!(1, 1))),
        Ok(Token::space(location!(3, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(4, 1))),
        Ok(Token::newline(location!(7, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("fi", location!(1, 4))),
        Ok(Token::space(location!(3, 4))),
        Ok(Token::write_to(location!(4, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(6, 4))),
        Ok(Token::newline(location!(9, 4)))
    );

    assert_lex!(
        indoc! {r#"
          if foo
          then
            bar
          fi &
        "#},
        Ok(Token::keyword("if", location!(1, 1))),
        Ok(Token::space(location!(3, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(4, 1))),
        Ok(Token::newline(location!(7, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("fi", location!(1, 4))),
        Ok(Token::space(location!(3, 4))),
        Ok(Token::background(location!(4, 4))),
        Ok(Token::newline(location!(5, 4)))
    );

    assert_lex!(
        indoc! {r#"
          if foo
          then
            bar
          fi | if baz
          then
            qux
          fi
        "#},
        Ok(Token::keyword("if", location!(1, 1))),
        Ok(Token::space(location!(3, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(4, 1))),
        Ok(Token::newline(location!(7, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("fi", location!(1, 4))),
        Ok(Token::space(location!(3, 4))),
        Ok(Token::pipe(location!(4, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::keyword("if", location!(6, 4))),
        Ok(Token::space(location!(8, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(9, 4))),
        Ok(Token::newline(location!(12, 4))),
        Ok(Token::keyword("then", location!(1, 5))),
        Ok(Token::newline(location!(5, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("fi", location!(1, 7))),
        Ok(Token::newline(location!(3, 7)))
    );
}

#[test]
fn it_unless_statement() {
    assert_lex!(
        "unless foo; then bar; end",
        Ok(Token::keyword("unless", location!(1, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1))),
        Ok(Token::termination(location!(11, 1))),
        Ok(Token::space(location!(12, 1))),
        Ok(Token::keyword("then", location!(13, 1))),
        Ok(Token::space(location!(17, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(18, 1))),
        Ok(Token::termination(location!(21, 1))),
        Ok(Token::space(location!(22, 1))),
        Ok(Token::keyword("end", location!(23, 1)))
    );

    assert_lex!(
        indoc! {r#"
          unless foo
            bar
          else
            grault
          end
        "#},
        Ok(Token::keyword("unless", location!(1, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1))),
        Ok(Token::newline(location!(11, 1))),
        Ok(Token::space(location!(1, 2))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 2))),
        Ok(Token::newline(location!(6, 2))),
        Ok(Token::keyword("else", location!(1, 3))),
        Ok(Token::newline(location!(5, 3))),
        Ok(Token::space(location!(1, 4))),
        Ok(Token::word("grault", WordKind::Normal, location!(3, 4))),
        Ok(Token::newline(location!(9, 4))),
        Ok(Token::keyword("end", location!(1, 5))),
        Ok(Token::newline(location!(4, 5)))
    );

    assert_lex!(
        indoc! {r#"
          unless foo
          then if bar
              baz
            fi
          else
            qux
          end
        "#},
        Ok(Token::keyword("unless", location!(1, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1))),
        Ok(Token::newline(location!(11, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::space(location!(5, 2))),
        Ok(Token::keyword("if", location!(6, 2))),
        Ok(Token::space(location!(8, 2))),
        Ok(Token::word("bar", WordKind::Normal, location!(9, 2))),
        Ok(Token::newline(location!(12, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("baz", WordKind::Normal, location!(5, 3))),
        Ok(Token::newline(location!(8, 3))),
        Ok(Token::space(location!(1, 4))),
        Ok(Token::keyword("fi", location!(3, 4))),
        Ok(Token::newline(location!(5, 4))),
        Ok(Token::keyword("else", location!(1, 5))),
        Ok(Token::newline(location!(5, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("end", location!(1, 7))),
        Ok(Token::newline(location!(4, 7)))
    );

    assert_lex!(
        indoc! {r#"
          unless if foo
          then
            bar
          fi
          then
            baz
          else
            qux
          end
        "#},
        Ok(Token::keyword("unless", location!(1, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::keyword("if", location!(8, 1))),
        Ok(Token::space(location!(10, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(11, 1))),
        Ok(Token::newline(location!(14, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("fi", location!(1, 4))),
        Ok(Token::newline(location!(3, 4))),
        Ok(Token::keyword("then", location!(1, 5))),
        Ok(Token::newline(location!(5, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("baz", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("else", location!(1, 7))),
        Ok(Token::newline(location!(5, 7))),
        Ok(Token::space(location!(1, 8))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 8))),
        Ok(Token::newline(location!(6, 8))),
        Ok(Token::keyword("end", location!(1, 9))),
        Ok(Token::newline(location!(4, 9)))
    );

    assert_lex!(
        indoc! {r#"
          unless foo
          then
            bar
          end > baz
        "#},
        Ok(Token::keyword("unless", location!(1, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1))),
        Ok(Token::newline(location!(11, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("end", location!(1, 4))),
        Ok(Token::space(location!(4, 4))),
        Ok(Token::write_to(location!(5, 4))),
        Ok(Token::space(location!(6, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(7, 4))),
        Ok(Token::newline(location!(10, 4)))
    );

    assert_lex!(
        indoc! {r#"
          unless foo
          then
            bar
          end &
        "#},
        Ok(Token::keyword("unless", location!(1, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1))),
        Ok(Token::newline(location!(11, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("end", location!(1, 4))),
        Ok(Token::space(location!(4, 4))),
        Ok(Token::background(location!(5, 4))),
        Ok(Token::newline(location!(6, 4)))
    );

    assert_lex!(
        indoc! {r#"
          unless foo
          then
            bar
          end | unless baz
          then
            qux
          end
        "#},
        Ok(Token::keyword("unless", location!(1, 1))),
        Ok(Token::space(location!(7, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 1))),
        Ok(Token::newline(location!(11, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("end", location!(1, 4))),
        Ok(Token::space(location!(4, 4))),
        Ok(Token::pipe(location!(5, 4))),
        Ok(Token::space(location!(6, 4))),
        Ok(Token::keyword("unless", location!(7, 4))),
        Ok(Token::space(location!(13, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(14, 4))),
        Ok(Token::newline(location!(17, 4))),
        Ok(Token::keyword("then", location!(1, 5))),
        Ok(Token::newline(location!(5, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("end", location!(1, 7))),
        Ok(Token::newline(location!(4, 7)))
    );
}

#[test]
fn it_while_statement() {
    assert_lex!(
        "while foo; do bar; done",
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::termination(location!(10, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::keyword("do", location!(12, 1))),
        Ok(Token::space(location!(14, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(15, 1))),
        Ok(Token::termination(location!(18, 1))),
        Ok(Token::space(location!(19, 1))),
        Ok(Token::keyword("done", location!(20, 1)))
    );

    assert_lex!(
        indoc! {r#"
          while foo
          do
            bar
          done
        "#},
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::newline(location!(5, 4)))
    );

    assert_lex!(
        indoc! {r#"
          while foo
            bar
          end
        "#},
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::space(location!(1, 2))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 2))),
        Ok(Token::newline(location!(6, 2))),
        Ok(Token::keyword("end", location!(1, 3))),
        Ok(Token::newline(location!(4, 3)))
    );

    assert_lex!(
        indoc! {r#"
          while while foo
          do
            bar
          done
          do
            baz
          done
        "#},
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::keyword("while", location!(7, 1))),
        Ok(Token::space(location!(12, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(13, 1))),
        Ok(Token::newline(location!(16, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::newline(location!(5, 4))),
        Ok(Token::keyword("do", location!(1, 5))),
        Ok(Token::newline(location!(3, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("baz", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("done", location!(1, 7))),
        Ok(Token::newline(location!(5, 7)))
    );

    assert_lex!(
        indoc! {r#"
          while if foo
          then
            bar
          fi
          do
            baz
          done
        "#},
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::keyword("if", location!(7, 1))),
        Ok(Token::space(location!(9, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(10, 1))),
        Ok(Token::newline(location!(13, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("fi", location!(1, 4))),
        Ok(Token::newline(location!(3, 4))),
        Ok(Token::keyword("do", location!(1, 5))),
        Ok(Token::newline(location!(3, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("baz", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("done", location!(1, 7))),
        Ok(Token::newline(location!(5, 7)))
    );

    assert_lex!(
        indoc! {r#"
          while foo
          do while bar
          do baz
          done done
        "#},
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::space(location!(3, 2))),
        Ok(Token::keyword("while", location!(4, 2))),
        Ok(Token::space(location!(9, 2))),
        Ok(Token::word("bar", WordKind::Normal, location!(10, 2))),
        Ok(Token::newline(location!(13, 2))),
        Ok(Token::keyword("do", location!(1, 3))),
        Ok(Token::space(location!(3, 3))),
        Ok(Token::word("baz", WordKind::Normal, location!(4, 3))),
        Ok(Token::newline(location!(7, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::keyword("done", location!(6, 4))),
        Ok(Token::newline(location!(10, 4)))
    );

    assert_lex!(
        indoc! {r#"
          while foo
          do
            bar
          done > baz
        "#},
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::write_to(location!(6, 4))),
        Ok(Token::space(location!(7, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(8, 4))),
        Ok(Token::newline(location!(11, 4)))
    );

    assert_lex!(
        indoc! {r#"
          while foo
          do
            bar
          done &
        "#},
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::background(location!(6, 4))),
        Ok(Token::newline(location!(7, 4)))
    );

    assert_lex!(
        indoc! {r#"
          while foo
          do
            bar
          done | while baz
          do
            qux
          done
        "#},
        Ok(Token::keyword("while", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::pipe(location!(6, 4))),
        Ok(Token::space(location!(7, 4))),
        Ok(Token::keyword("while", location!(8, 4))),
        Ok(Token::space(location!(13, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(14, 4))),
        Ok(Token::newline(location!(17, 4))),
        Ok(Token::keyword("do", location!(1, 5))),
        Ok(Token::newline(location!(3, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("done", location!(1, 7))),
        Ok(Token::newline(location!(5, 7)))
    );
}

#[test]
fn it_until_statement() {
    assert_lex!(
        "until foo; do bar; done",
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::termination(location!(10, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::keyword("do", location!(12, 1))),
        Ok(Token::space(location!(14, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(15, 1))),
        Ok(Token::termination(location!(18, 1))),
        Ok(Token::space(location!(19, 1))),
        Ok(Token::keyword("done", location!(20, 1)))
    );

    assert_lex!(
        indoc! {r#"
          until foo
          do
            bar
          done
        "#},
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::newline(location!(5, 4)))
    );

    assert_lex!(
        indoc! {r#"
          until foo
            bar
          end
        "#},
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::space(location!(1, 2))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 2))),
        Ok(Token::newline(location!(6, 2))),
        Ok(Token::keyword("end", location!(1, 3))),
        Ok(Token::newline(location!(4, 3)))
    );

    assert_lex!(
        indoc! {r#"
          until until foo
          do
            bar
          done
          do
            baz
          done
        "#},
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::keyword("until", location!(7, 1))),
        Ok(Token::space(location!(12, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(13, 1))),
        Ok(Token::newline(location!(16, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::newline(location!(5, 4))),
        Ok(Token::keyword("do", location!(1, 5))),
        Ok(Token::newline(location!(3, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("baz", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("done", location!(1, 7))),
        Ok(Token::newline(location!(5, 7)))
    );

    assert_lex!(
        indoc! {r#"
          until if foo
          then
            bar
          fi
          do
            baz
          done
        "#},
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::keyword("if", location!(7, 1))),
        Ok(Token::space(location!(9, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(10, 1))),
        Ok(Token::newline(location!(13, 1))),
        Ok(Token::keyword("then", location!(1, 2))),
        Ok(Token::newline(location!(5, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("fi", location!(1, 4))),
        Ok(Token::newline(location!(3, 4))),
        Ok(Token::keyword("do", location!(1, 5))),
        Ok(Token::newline(location!(3, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("baz", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("done", location!(1, 7))),
        Ok(Token::newline(location!(5, 7)))
    );

    assert_lex!(
        indoc! {r#"
          until foo
          do until bar
          do baz
          done done
        "#},
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::space(location!(3, 2))),
        Ok(Token::keyword("until", location!(4, 2))),
        Ok(Token::space(location!(9, 2))),
        Ok(Token::word("bar", WordKind::Normal, location!(10, 2))),
        Ok(Token::newline(location!(13, 2))),
        Ok(Token::keyword("do", location!(1, 3))),
        Ok(Token::space(location!(3, 3))),
        Ok(Token::word("baz", WordKind::Normal, location!(4, 3))),
        Ok(Token::newline(location!(7, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::keyword("done", location!(6, 4))),
        Ok(Token::newline(location!(10, 4)))
    );

    assert_lex!(
        indoc! {r#"
          until foo
          do
            bar
          done > baz
        "#},
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::write_to(location!(6, 4))),
        Ok(Token::space(location!(7, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(8, 4))),
        Ok(Token::newline(location!(11, 4)))
    );

    assert_lex!(
        indoc! {r#"
          until foo
          do
            bar
          done &
        "#},
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::background(location!(6, 4))),
        Ok(Token::newline(location!(7, 4)))
    );

    assert_lex!(
        indoc! {r#"
          until foo
          do
            bar
          done | until baz
          do
            qux
          done
        "#},
        Ok(Token::keyword("until", location!(1, 1))),
        Ok(Token::space(location!(6, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(7, 1))),
        Ok(Token::newline(location!(10, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::pipe(location!(6, 4))),
        Ok(Token::space(location!(7, 4))),
        Ok(Token::keyword("until", location!(8, 4))),
        Ok(Token::space(location!(13, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(14, 4))),
        Ok(Token::newline(location!(17, 4))),
        Ok(Token::keyword("do", location!(1, 5))),
        Ok(Token::newline(location!(3, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("done", location!(1, 7))),
        Ok(Token::newline(location!(5, 7)))
    );
}

#[test]
fn for_statement() {
    assert_lex!(
        "for foo in bar baz; do qux; done",
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::keyword("in", location!(9, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(12, 1))),
        Ok(Token::space(location!(15, 1))),
        Ok(Token::word("baz", WordKind::Normal, location!(16, 1))),
        Ok(Token::termination(location!(19, 1))),
        Ok(Token::space(location!(20, 1))),
        Ok(Token::keyword("do", location!(21, 1))),
        Ok(Token::space(location!(23, 1))),
        Ok(Token::word("qux", WordKind::Normal, location!(24, 1))),
        Ok(Token::termination(location!(27, 1))),
        Ok(Token::space(location!(28, 1))),
        Ok(Token::keyword("done", location!(29, 1)))
    );

    assert_lex!(
        "for foo; do bar; done",
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::termination(location!(8, 1))),
        Ok(Token::space(location!(9, 1))),
        Ok(Token::keyword("do", location!(10, 1))),
        Ok(Token::space(location!(12, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(13, 1))),
        Ok(Token::termination(location!(16, 1))),
        Ok(Token::space(location!(17, 1))),
        Ok(Token::keyword("done", location!(18, 1)))
    );

    assert_lex!(
        indoc! {r#"
        for foo in bar baz
        do
          qux
        done
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::keyword("in", location!(9, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(12, 1))),
        Ok(Token::space(location!(15, 1))),
        Ok(Token::word("baz", WordKind::Normal, location!(16, 1))),
        Ok(Token::newline(location!(19, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::newline(location!(5, 4)))
    );

    assert_lex!(
        indoc! {r#"
        for foo in bar baz
          qux
        end
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::keyword("in", location!(9, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(12, 1))),
        Ok(Token::space(location!(15, 1))),
        Ok(Token::word("baz", WordKind::Normal, location!(16, 1))),
        Ok(Token::newline(location!(19, 1))),
        Ok(Token::space(location!(1, 2))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 2))),
        Ok(Token::newline(location!(6, 2))),
        Ok(Token::keyword("end", location!(1, 3))),
        Ok(Token::newline(location!(4, 3)))
    );
    assert_lex!(
        indoc! {r#"
        for foo in bar baz
        do for qux
        do quux
        done done
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::keyword("in", location!(9, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(12, 1))),
        Ok(Token::space(location!(15, 1))),
        Ok(Token::word("baz", WordKind::Normal, location!(16, 1))),
        Ok(Token::newline(location!(19, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::space(location!(3, 2))),
        Ok(Token::keyword("for", location!(4, 2))),
        Ok(Token::space(location!(7, 2))),
        Ok(Token::word("qux", WordKind::Normal, location!(8, 2))),
        Ok(Token::newline(location!(11, 2))),
        Ok(Token::keyword("do", location!(1, 3))),
        Ok(Token::space(location!(3, 3))),
        Ok(Token::word("quux", WordKind::Normal, location!(4, 3))),
        Ok(Token::newline(location!(8, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::keyword("done", location!(6, 4))),
        Ok(Token::newline(location!(10, 4)))
    );

    assert_lex!(
        indoc! {r#"
        for foo"bar"'baz'
        do
          qux
        done
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(9, 1))),
        Ok(Token::word("baz", WordKind::Quote, location!(13, 1))),
        Ok(Token::newline(location!(18, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::newline(location!(5, 4)))
    );

    assert_lex!(
        indoc! {r#"
        for foo in "bar" 'baz' `qux`
        do
          quux
        done
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::keyword("in", location!(9, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(13, 1))),
        Ok(Token::space(location!(17, 1))),
        Ok(Token::word("baz", WordKind::Quote, location!(18, 1))),
        Ok(Token::space(location!(23, 1))),
        Ok(Token::word("qux", WordKind::Command, location!(24, 1))),
        Ok(Token::newline(location!(29, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("quux", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(7, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::newline(location!(5, 4)))
    );

    assert_lex!(
        indoc! {r#"
        for foo in bar
        do if baz
        then
          qux
        fi done
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::space(location!(8, 1))),
        Ok(Token::keyword("in", location!(9, 1))),
        Ok(Token::space(location!(11, 1))),
        Ok(Token::word("bar", WordKind::Normal, location!(12, 1))),
        Ok(Token::newline(location!(15, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::space(location!(3, 2))),
        Ok(Token::keyword("if", location!(4, 2))),
        Ok(Token::space(location!(6, 2))),
        Ok(Token::word("baz", WordKind::Normal, location!(7, 2))),
        Ok(Token::newline(location!(10, 2))),
        Ok(Token::keyword("then", location!(1, 3))),
        Ok(Token::newline(location!(5, 3))),
        Ok(Token::space(location!(1, 4))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 4))),
        Ok(Token::newline(location!(6, 4))),
        Ok(Token::keyword("fi", location!(1, 5))),
        Ok(Token::space(location!(3, 5))),
        Ok(Token::keyword("done", location!(4, 5))),
        Ok(Token::newline(location!(8, 5)))
    );

    assert_lex!(
        indoc! {r#"
        for foo
        do
          bar
        done > baz
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::newline(location!(8, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::write_to(location!(6, 4))),
        Ok(Token::space(location!(7, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(8, 4))),
        Ok(Token::newline(location!(11, 4)))
    );

    assert_lex!(
        indoc! {r#"
        for foo
        do
          bar
        done &
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::newline(location!(8, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::background(location!(6, 4))),
        Ok(Token::newline(location!(7, 4)))
    );

    assert_lex!(
        indoc! {r#"
        for foo
        do
          bar
        done | foo baz
        do
          qux
        done
        "#},
        Ok(Token::keyword("for", location!(1, 1))),
        Ok(Token::space(location!(4, 1))),
        Ok(Token::word("foo", WordKind::Normal, location!(5, 1))),
        Ok(Token::newline(location!(8, 1))),
        Ok(Token::keyword("do", location!(1, 2))),
        Ok(Token::newline(location!(3, 2))),
        Ok(Token::space(location!(1, 3))),
        Ok(Token::word("bar", WordKind::Normal, location!(3, 3))),
        Ok(Token::newline(location!(6, 3))),
        Ok(Token::keyword("done", location!(1, 4))),
        Ok(Token::space(location!(5, 4))),
        Ok(Token::pipe(location!(6, 4))),
        Ok(Token::space(location!(7, 4))),
        Ok(Token::word("foo", WordKind::Normal, location!(8, 4))),
        Ok(Token::space(location!(11, 4))),
        Ok(Token::word("baz", WordKind::Normal, location!(12, 4))),
        Ok(Token::newline(location!(15, 4))),
        Ok(Token::keyword("do", location!(1, 5))),
        Ok(Token::newline(location!(3, 5))),
        Ok(Token::space(location!(1, 6))),
        Ok(Token::word("qux", WordKind::Normal, location!(3, 6))),
        Ok(Token::newline(location!(6, 6))),
        Ok(Token::keyword("done", location!(1, 7))),
        Ok(Token::newline(location!(5, 7)))
    );
}
