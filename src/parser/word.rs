use super::{Annotate, ErrorKind, ParseResult, ParserError, Span};
use crate::syntax_check;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, none_of, one_of},
    combinator::{map, peek, verify},
    multi::{fold_many0, fold_many1, many0},
    sequence::{delimited, preceded, terminated, tuple},
};

pub type Word = Annotate<WordKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum WordKind {
    Literal(String),
    Quote(String),
    Variable(String),
}

pub fn parse_word(i: Span) -> ParseResult<Vec<Word>> {
    fold_many1(
        alt((parse_percent_word, parse_quoted_word, parse_normal_word)),
        Vec::new,
        |mut v, words| {
            for word in words {
                v.push(word);
            }
            v
        },
    )(i)
}

fn parse_percent_word(i: Span) -> ParseResult<Vec<Word>> {
    const SYMBOL: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    peek(char('%'))(i)?;

    let kind = one_of("qQ");
    let kind_parser = alt((
        map(one_of(SYMBOL), |c| ('Q', c)),
        map(tuple((kind, one_of(SYMBOL))), |(kind, symbol)| {
            (kind, symbol)
        }),
    ));
    let (o, (kind, left)) =
        syntax_check!(i, preceded(char('%'), kind_parser), ErrorKind::UnknownType)?;
    let right = map_bracket(left);
    let quote = match kind {
        'q' => true,
        _ => false,
    };

    let fragment = move |i| parse_word_fragment(i, &*right.to_string(), true, quote);
    syntax_check!(
        o,
        terminated(fragment, char(right)),
        ErrorKind::Unterminated
    )
}

fn map_bracket(s: char) -> char {
    match s {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => s,
    }
}

fn parse_quoted_word(i: Span) -> ParseResult<Vec<Word>> {
    let (i, left) = one_of("'\"")(i)?;
    let term = left.to_string();
    let quote = match left {
        '\'' => true,
        _ => false,
    };

    let fragment = move |i| parse_word_fragment(i, &*term, true, quote);
    syntax_check!(i, terminated(fragment, char(left)), ErrorKind::Unterminated)
}

fn parse_normal_word(i: Span) -> ParseResult<Vec<Word>> {
    let parser = |i| parse_word_fragment(i, " \t\r\n\"'", false, false);
    verify(parser, |v: &Vec<_>| !v.is_empty())(i)
}

fn parse_word_fragment<'a>(
    i: Span<'a>,
    term: &str,
    backslash: bool,
    quote: bool,
) -> ParseResult<'a, Vec<Word>> {
    let literal = |i| {
        let parser = |i| parse_literal_string(none_of(term), backslash, one_of(term), quote)(i);
        let (o, s) = verify(parser, |s: &str| !s.is_empty())(i)?;
        let w = Word {
            value: WordKind::Literal(s),
            column: i.get_utf8_column(),
            line: i.location_line(),
        };
        Ok((o, w))
    };
    let variable = |i| {
        let (o, s) = verify(parse_variable_string, |_: &str| !quote)(i)?;
        let w = Word {
            value: WordKind::Variable(s),
            column: i.get_utf8_column(),
            line: i.location_line(),
        };
        Ok((o, w))
    };

    many0(alt((variable, literal)))(i)
}

fn parse_literal_string<'a, F, G>(
    pattern: F,
    backslash: bool,
    escape: G,
    quote: bool,
) -> impl FnMut(Span<'a>) -> ParseResult<String>
where
    F: nom::Parser<Span<'a>, char, ParserError<'a>>,
    G: nom::Parser<Span<'a>, char, ParserError<'a>>,
{
    let literal = fold_many1(
        verify(pattern, move |s| match s {
            '\\' => false,
            '$' => quote,
            _ => true,
        }),
        String::new,
        |mut string, c| {
            string.push(c);
            string
        },
    );
    let escaped_space = preceded(char('\\'), escape);
    let fragment = alt((
        literal,
        map(escaped_space, |c| c.to_string()),
        map(char('\\'), move |_| {
            if backslash { "\\" } else { "" }.to_string()
        }),
    ));

    fold_many0(fragment, String::new, |mut s, f| {
        s.push_str(&*f);
        s
    })
}

fn parse_variable_string(i: Span) -> ParseResult<String> {
    preceded(
        char('$'),
        map(
            alt((
                delimited(char('{'), is_not("}"), char('}')),
                tag("?"),
                tag("@"),
                tag("#"),
                tag("!"),
                is_not("\"#$%&'()*+,-./:;<=>?@[\\]^`{|}~! "),
            )),
            |s: Span| s.to_string(),
        ),
    )(i)
}

#[cfg(test)]
mod test {
    use super::super::ParserError;
    use super::*;
    use anyhow::Result;
    use nom::character::complete::line_ending;

    #[test]
    fn test_parse_word() -> Result<()> {
        let (s, got) = parse_word(Span::new("foo bar"))?;
        assert_eq!(1, got.len());
        assert_eq!(WordKind::Literal("foo".to_string()), got[0].value);
        assert_eq!(" bar", s.to_string());

        let (s, got) = parse_word(Span::new(r#"foo"bar"'baz' foo"#))?;
        assert_eq!(3, got.len());
        assert_eq!(WordKind::Literal("foo".to_string()), got[0].value);
        assert_eq!(WordKind::Literal("bar".to_string()), got[1].value);
        assert_eq!(WordKind::Literal("baz".to_string()), got[2].value);
        assert_eq!(" foo", s.to_string());

        let (s, got) = parse_word(Span::new(r#"foo"ba'r""b'az""#))?;
        assert_eq!(3, got.len());
        assert_eq!(WordKind::Literal("foo".to_string()), got[0].value);
        assert_eq!(WordKind::Literal("ba'r".to_string()), got[1].value);
        assert_eq!(WordKind::Literal("b'az".to_string()), got[2].value);
        assert_eq!("", s.to_string());

        Ok(())
    }

    #[test]
    fn test_parse_normal_word() -> Result<()> {
        let tests = vec![
            ("foobar", "foobar"),
            (" \t\r\na", "\\ \\\t\\\r\\\n\\a"),
            ("foo bar\nbaz", "foo\\ bar\\\nbaz"),
            (r#"foo barnbaz"#, r#"foo\ bar\nbaz"#),
        ];
        for (expect, test) in tests.iter() {
            let (s, got) = parse_normal_word(Span::new(test))?;
            assert_eq!(1, got.len());
            assert_eq!(WordKind::Literal(expect.to_string()), got[0].value);
            assert_eq!("", s.to_string());
        }

        let (s, got) = parse_normal_word(Span::new("foo\nbar"))?;
        assert_eq!(1, got.len());
        assert_eq!(WordKind::Literal("foo".to_string()), got[0].value);
        assert_eq!("\nbar", s.to_string());

        let (s, _) = line_ending::<_, ParserError>(s)?;
        let (s, got) = parse_normal_word(s)?;
        assert_eq!(WordKind::Literal("bar".to_string()), got[0].value);
        assert_eq!(1, got.len());
        assert_eq!(1, got[0].column);
        assert_eq!(2, got[0].line);
        assert_eq!("", s.to_string());

        let (s, got) = parse_normal_word(Span::new("foo$bar"))?;
        assert_eq!(2, got.len());
        assert_eq!(WordKind::Literal("foo".to_string()), got[0].value);
        assert_eq!(WordKind::Variable("bar".to_string()), got[1].value);
        assert_eq!("", s.to_string());

        assert!(parse_normal_word(Span::new("")).is_err());

        Ok(())
    }

    #[test]
    fn test_parse_percent_word() -> Result<()> {
        let tests = vec![
            ("foo!bar\\\nbaz", "%!foo\\!bar\\\nbaz!"),
            ("foo)bar", "%(foo\\)bar)"),
            ("foo}bar", "%{foo\\}bar}"),
            ("foo]bar", "%[foo\\]bar]"),
            ("foo>bar", "%<foo\\>bar>"),
            ("foo%bar", "%%foo\\%bar%"),
            ("foo)bar", "%Q(foo\\)bar)"),
            ("foo}bar", "%Q{foo\\}bar}"),
            ("foo]bar", "%Q[foo\\]bar]"),
            ("foo>bar", "%Q<foo\\>bar>"),
            ("foo!bar", "%Q!foo\\!bar!"),
            ("foo)bar", "%q(foo\\)bar)"),
            ("foo}bar", "%q{foo\\}bar}"),
            ("foo]bar", "%q[foo\\]bar]"),
            ("foo>bar", "%q<foo\\>bar>"),
            ("foo!bar", "%q!foo\\!bar!"),
            ("$foo", "%q!$foo!"),
        ];
        for (expect, test) in tests.iter() {
            let (s, got) = parse_percent_word(Span::new(test))?;
            assert_eq!(WordKind::Literal(expect.to_string()), got[0].value);
            assert_eq!("", s.to_string());
        }

        let (s, got) = parse_percent_word(Span::new("%!$foo!"))?;
        assert_eq!(1, got.len());
        assert_eq!(WordKind::Variable("foo".to_string()), got[0].value);
        assert_eq!("", s.to_string());

        assert!(parse_percent_word(Span::new("")).is_err());
        match parse_percent_word(Span::new("%afoo")) {
            Err(nom::Err::Failure(e)) => {
                assert_eq!(ErrorKind::UnknownType, e.kind);
            }
            _ => unreachable![],
        }
        match parse_percent_word(Span::new("%!foo")) {
            Err(nom::Err::Failure(e)) => {
                assert_eq!(ErrorKind::Unterminated, e.kind);
            }
            _ => unreachable![],
        }
        Ok(())
    }

    #[test]
    fn test_map_bracket() {
        let expects = vec![('(', ')'), ('{', '}'), ('[', ']'), ('<', '>'), ('!', '!')];
        for (test, expect) in expects.iter() {
            assert_eq!(*expect, map_bracket(*test));
        }
    }

    #[test]
    fn test_parse_quoted_word() -> Result<()> {
        let tests = vec![
            ("foo bar baz", r#""foo bar baz""#),
            ("foo bar baz", r#"'foo bar baz'"#),
            (r#"foo'bar\"baz"#, r#"'foo\'bar\"baz'"#),
            (r#"foo"bar\'baz"#, r#""foo\"bar\'baz""#),
            ("foo\nbar", "'foo\nbar'"),
            ("foo\nbar", "\"foo\nbar\""),
            ("$FOO", "'$FOO'"),
        ];

        for (expect, test) in tests.iter() {
            let (s, got) = parse_quoted_word(Span::new(test))?;
            assert_eq!(1, got.len());
            assert_eq!(WordKind::Literal(expect.to_string()), got[0].value);
            assert_eq!("", s.to_string());
        }

        let (s, got) = parse_quoted_word(Span::new(r#""$foo""#))?;
        assert_eq!(1, got.len());
        assert_eq!(WordKind::Variable("foo".to_string()), got[0].value);
        assert_eq!("", s.to_string());

        let (_, got) = parse_quoted_word(Span::new(r#""""#))?;
        assert_eq!(0, got.len());

        let (s, got) = parse_quoted_word(Span::new(r#""foo""bar""#))?;
        assert_eq!(WordKind::Literal("foo".to_string()), got[0].value);
        assert_eq!(r#""bar""#, s.to_string());

        assert!(parse_quoted_word(Span::new("")).is_err());
        match parse_quoted_word(Span::new(r#""foo"#)) {
            Err(nom::Err::Failure(e)) => {
                assert_eq!(ErrorKind::Unterminated, e.kind);
            }
            _ => unreachable![],
        }
        Ok(())
    }

    #[test]
    fn test_parse_variable_fragment() -> Result<()> {
        let (s, got) = parse_variable_string(Span::new("$FOO"))?;
        assert_eq!("FOO", &*got);
        assert_eq!("", s.to_string());

        let (s, got) = parse_variable_string(Span::new("$FOO "))?;
        assert_eq!("FOO", &*got);
        assert_eq!(" ", s.to_string());

        let (s, got) = parse_variable_string(Span::new("${FOO}"))?;
        assert_eq!("FOO", &*got);
        assert_eq!("", s.to_string());

        let (s, got) = parse_variable_string(Span::new("$???"))?;
        assert_eq!("?", &*got);
        assert_eq!("??", s.to_string());

        let (s, got) = parse_variable_string(Span::new("$@@@"))?;
        assert_eq!("@", &*got);
        assert_eq!("@@", s.to_string());

        let (s, got) = parse_variable_string(Span::new("$###"))?;
        assert_eq!("#", &*got);
        assert_eq!("##", s.to_string());

        Ok(())
    }
}
