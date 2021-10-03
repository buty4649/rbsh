use super::word::{parse_word, Word};
use super::{Annotate, ErrorKind, ParseResult, Span};
use crate::syntax_check;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit0, digit1, none_of, one_of, space0},
    combinator::{cut, map, opt, peek},
    sequence::{preceded, separated_pair, terminated, tuple},
};

pub type Fdsize = u16;
pub type Redirect = Annotate<RedirectKind>;

#[derive(Clone, Debug, PartialEq)]
pub enum RedirectKind {
    ReadFrom(Fdsize, Vec<Word>),     // fd filename
    WriteTo(Fdsize, Vec<Word>),      // fd filename
    WriteBoth(Vec<Word>),            // filename
    ReadCopy(Fdsize, Fdsize, bool),  // fd(src) fd(dest) close?
    WriteCopy(Fdsize, Fdsize, bool), // fd(src) fd(dest) close?
    Append(Fdsize, Vec<Word>),       // fd filename
    AppendBoth(Vec<Word>),           // fd filename
    Close(Fdsize),                   // fd
    ReadWrite(Fdsize, Vec<Word>),    // fd filename
}

pub fn parse_redirect(i: Span) -> ParseResult<Redirect> {
    let (o, r) = alt((
        parse_redirect_close,
        parse_redirect_append,
        parse_redirect_copy,
        parse_redirect_rw,
    ))(i)?;

    Ok((
        o,
        Redirect {
            value: r,
            column: i.get_utf8_column(),
            line: i.location_line(),
        },
    ))
}

fn parse_redirect_rw(i: Span) -> ParseResult<RedirectKind> {
    // ReadForm: < word , n< word
    // WriteTo: > word, n> word
    // WriteBoth: &> word, >& word
    // ReadWrite: <> word, n<> word
    alt((
        map(
            separated_pair(alt((tag("&>"), tag(">&"))), valid_sperator, parse_word),
            |(_, word)| RedirectKind::WriteBoth(word),
        ),
        map(
            separated_pair(tuple((digit0, tag("<>"))), valid_sperator, parse_word),
            |((fd, _), word)| {
                let fd = fd.parse::<Fdsize>().unwrap_or(0);
                RedirectKind::ReadWrite(fd, word)
            },
        ),
        map(
            separated_pair(tuple((digit0, one_of("<>"))), valid_sperator, parse_word),
            |((digit, rw), words)| {
                let digit = digit.parse::<Fdsize>().unwrap_or(default_fd(rw));
                match rw {
                    '<' => RedirectKind::ReadFrom(digit, words),
                    '>' => RedirectKind::WriteTo(digit, words),
                    _ => unreachable![],
                }
            },
        ),
    ))(i)
}

fn parse_redirect_copy(i: Span) -> ParseResult<RedirectKind> {
    // ReadCopy: <&n, n<&n-
    // WriteCopy: >&n, n>&n-
    let (o, (src, rw)) = map(
        terminated(tuple((digit0, one_of("<>"))), char('&')),
        |(src, rw): (Span, char)| {
            let src = src.parse::<Fdsize>().unwrap_or(default_fd(rw));
            (src, rw)
        },
    )(i)?;
    let (o, (dest, close)) = syntax_check!(
        o,
        map(
            tuple((digit1, opt(char('-')))),
            |(dest, close): (Span, Option<char>)| {
                let dest = dest.parse::<Fdsize>().unwrap();
                let close = match close {
                    Some(_) => true,
                    None => false,
                };
                (dest, close)
            },
        ),
        ErrorKind::InvalidFileDescriptor
    )?;
    let r = match rw {
        '<' => RedirectKind::ReadCopy(src, dest, close),
        '>' => RedirectKind::WriteCopy(src, dest, close),
        _ => unreachable![],
    };
    Ok((o, r))
}

fn parse_redirect_close(i: Span) -> ParseResult<RedirectKind> {
    // Close: <&-, n<&-, >&-, n>&-
    map(
        terminated(tuple((digit0, one_of("<>"))), tag("&-")),
        |(digit, rw): (Span, char)| {
            let digit = digit.parse::<Fdsize>().unwrap_or(default_fd(rw));
            RedirectKind::Close(digit)
        },
    )(i)
}

fn parse_redirect_append(i: Span) -> ParseResult<RedirectKind> {
    // Append: >> word, n>> word
    // AppendBoth: &>> word
    alt((
        map(
            preceded(tuple((tag("&>>"), valid_sperator)), parse_word),
            |word| RedirectKind::AppendBoth(word),
        ),
        map(
            separated_pair(tuple((digit0, tag(">>"))), valid_sperator, parse_word),
            |((dest, _), word)| {
                let dest = dest.parse::<Fdsize>().unwrap_or(1);
                RedirectKind::Append(dest, word)
            },
        ),
    ))(i)
}

fn default_fd(rw: char) -> Fdsize {
    match rw {
        '<' => 0,
        '>' => 1,
        _ => unreachable![],
    }
}

fn valid_sperator(i: Span) -> ParseResult<Span> {
    cut(peek(none_of("<>&")))(i)?;
    space0(i)
}

#[cfg(test)]
mod test {
    use super::super::word::WordKind;
    use super::super::ParserError;
    use super::*;
    use anyhow::Result;
    use nom::character::complete::space1;

    #[test]
    fn test_parse_redirect() -> Result<()> {
        let (s, got) = parse_redirect(Span::new("> \"foo bar\" 2>&1 3>> 'baz'\"foo\""))?;
        if let RedirectKind::WriteTo(fd, word) = got.value {
            assert_eq!(1, fd);
            assert_eq!(1, word.len());
            assert_eq!(WordKind::Literal("foo bar".to_string()), word[0].value);
            assert_eq!(1, got.column);
            assert_eq!(1, got.line);
        } else {
            unreachable![]
        }
        assert_eq!(" 2>&1 3>> 'baz'\"foo\"", *s.fragment());

        let (s, _) = space1::<_, ParserError>(s)?;
        let (s, got) = parse_redirect(s)?;
        if let RedirectKind::WriteCopy(src, dest, close) = got.value {
            assert_eq!(2, src);
            assert_eq!(1, dest);
            assert_eq!(false, close);
            assert_eq!(13, got.column);
            assert_eq!(1, got.line);
        } else {
            unreachable![]
        }
        assert_eq!(" 3>> 'baz'\"foo\"", *s.fragment());

        let (s, _) = space1::<_, ParserError>(s)?;
        let (s, got) = parse_redirect(s)?;
        if let RedirectKind::Append(fd, word) = got.value {
            assert_eq!(3, fd);
            assert_eq!(2, word.len());
            assert_eq!(WordKind::Literal("baz".to_string()), word[0].value);
            assert_eq!(WordKind::Literal("foo".to_string()), word[1].value);
            assert_eq!(18, got.column);
            assert_eq!(1, got.line);
        } else {
            unreachable![]
        }
        assert_eq!("", *s.fragment());
        Ok(())
    }

    #[test]
    fn test_parse_redirect_rw() -> Result<()> {
        let expects = vec![
            (0, "foobar", "< foobar"),
            (0, "foobar", "<foobar"),
            (3, "foobar", "3< foobar"),
            (3, "foobar", "3<foobar"),
        ];
        for (fd, word, test) in expects {
            let (s, got) = parse_redirect_rw(Span::new(test))?;
            if let RedirectKind::ReadFrom(f, w) = got {
                assert_eq!(fd, f);
                assert_eq!(1, w.len());
                assert_eq!(WordKind::Literal(word.to_string()), w[0].value);
                assert_eq!("", s.to_string());
            } else {
                unreachable![]
            };
        }

        let expects = vec![
            (1, "foobar", "> foobar"),
            (1, "foobar", ">foobar"),
            (4, "foobar", "4> foobar"),
            (4, "foobar", "4>foobar"),
        ];
        for (fd, word, test) in expects {
            let (s, got) = parse_redirect_rw(Span::new(test))?;
            if let RedirectKind::WriteTo(f, w) = got {
                assert_eq!(fd, f);
                assert_eq!(1, w.len());
                assert_eq!(WordKind::Literal(word.to_string()), w[0].value);
                assert_eq!("", s.to_string());
            } else {
                unreachable![]
            };
        }

        let expects = vec![
            ("foobar", "&> foobar"),
            ("foobar", "&>foobar"),
            ("foobar", ">& foobar"),
            ("foobar", ">&foobar"),
        ];
        for (word, test) in expects {
            let (s, got) = parse_redirect_rw(Span::new(test))?;
            if let RedirectKind::WriteBoth(w) = got {
                assert_eq!(1, w.len());
                assert_eq!(WordKind::Literal(word.to_string()), w[0].value);
                assert_eq!("", s.to_string());
            } else {
                unreachable![]
            };
        }

        let expects = vec![
            (0, "foobar", "<> foobar"),
            (0, "foobar", "<>foobar"),
            (3, "foobar", "3<> foobar"),
            (3, "foobar", "3<>foobar"),
        ];
        for (fd, word, test) in expects {
            let (s, got) = parse_redirect_rw(Span::new(test))?;
            if let RedirectKind::ReadWrite(f, w) = got {
                assert_eq!(fd, f);
                assert_eq!(1, w.len());
                assert_eq!(WordKind::Literal(word.to_string()), w[0].value);
                assert_eq!("", s.to_string());
            } else {
                unreachable![]
            };
        }

        let (s, got) = parse_redirect_rw(Span::new("< 'foo bar'\"baz\""))?;
        if let RedirectKind::ReadFrom(f, w) = got {
            assert_eq!(0, f);
            assert_eq!(2, w.len());
            assert_eq!(WordKind::Literal("foo bar".to_string()), w[0].value);
            assert_eq!(WordKind::Literal("baz".to_string()), w[1].value);
            assert_eq!("", s.to_string());
        } else {
            unreachable![]
        };

        match parse_redirect_rw(Span::new("<&&foo")) {
            Err(nom::Err::Failure(e)) => {
                assert_eq!(ErrorKind::UnexpectedToken, e.kind);
            }
            _ => unreachable![],
        }

        match parse_redirect_rw(Span::new("<& > foo")) {
            Err(nom::Err::Failure(e)) => {
                assert_eq!(ErrorKind::UnexpectedToken, e.kind);
            }
            _ => unreachable![],
        }
        Ok(())
    }

    #[test]
    fn test_parse_redirect_copy() -> Result<()> {
        let expects = vec![
            (0, 3, false, "<&3"),
            (4, 3, false, "4<&3"),
            (0, 3, true, "<&3-"),
            (4, 3, true, "4<&3-"),
        ];
        for (src, dest, close, test) in expects {
            let (o, got) = parse_redirect_copy(Span::new(test))?;
            if let RedirectKind::ReadCopy(s, d, c) = got {
                assert_eq!(src, s);
                assert_eq!(dest, d);
                assert_eq!(close, c);
                assert_eq!("", o.to_string());
            } else {
                unreachable![]
            };
        }

        let expects = vec![
            (1, 3, false, ">&3"),
            (3, 4, false, "3>&4"),
            (1, 3, true, ">&3-"),
            (3, 4, true, "3>&4-"),
        ];
        for (src, dest, close, test) in expects {
            let (o, got) = parse_redirect_copy(Span::new(test))?;
            if let RedirectKind::WriteCopy(s, d, c) = got {
                assert_eq!(src, s);
                assert_eq!(dest, d);
                assert_eq!(close, c);
                assert_eq!("", o.to_string());
            } else {
                unreachable![]
            };
        }

        Ok(())
    }

    #[test]
    fn test_parse_redirect_close() -> Result<()> {
        let expects = vec![
            (RedirectKind::Close(0), "<&-"),
            (RedirectKind::Close(1), ">&-"),
            (RedirectKind::Close(3), "3<&-"),
            (RedirectKind::Close(4), "4>&-"),
        ];
        for (expect, test) in expects {
            let (s, got) = parse_redirect_close(Span::new(test))?;
            assert_eq!(expect, got);
            assert_eq!("", s.to_string());
        }
        Ok(())
    }

    #[test]
    fn test_prase_redirect_append() -> Result<()> {
        let expects = vec![
            (1, "foobar", ">> foobar"),
            (1, "foobar", ">>foobar"),
            (4, "foobar", "4>> foobar"),
            (4, "foobar", "4>>foobar"),
        ];
        for (fd, word, test) in expects {
            let (s, got) = parse_redirect_append(Span::new(test))?;
            if let RedirectKind::Append(f, w) = got {
                assert_eq!(fd, f);
                assert_eq!(1, w.len());
                assert_eq!(WordKind::Literal(word.to_string()), w[0].value);
                assert_eq!("", s.to_string());
            } else {
                unreachable![]
            };
        }

        let expects = vec![("foobar", "&>> foobar"), ("foobar", "&>>foobar")];
        for (word, test) in expects {
            let (s, got) = parse_redirect_append(Span::new(test))?;
            if let RedirectKind::AppendBoth(w) = got {
                assert_eq!(1, w.len());
                assert_eq!(WordKind::Literal(word.to_string()), w[0].value);
                assert_eq!("", s.to_string());
            } else {
                unreachable![]
            };
        }

        let (s, got) = parse_redirect_append(Span::new(">> 'foo bar'\"baz\""))?;
        if let RedirectKind::Append(f, w) = got {
            assert_eq!(1, f);
            assert_eq!(2, w.len());
            assert_eq!(WordKind::Literal("foo bar".to_string()), w[0].value);
            assert_eq!(WordKind::Literal("baz".to_string()), w[1].value);
            assert_eq!("", s.to_string());
        } else {
            unreachable![]
        };

        Ok(())
    }

    #[test]
    fn test_default_fd() {
        assert_eq!(0, default_fd('<'));
        assert_eq!(1, default_fd('>'));
    }
}
