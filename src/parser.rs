mod redirect;
mod word;

use nom::{
  branch::alt,
  character::complete::space1,
  combinator::map,
  multi::{many1, separated_list0},
  Finish, IResult,
};
use nom_locate::LocatedSpan;
use redirect::{parse_redirect, Redirect};
use word::{parse_word, Word};

pub type Span<'a> = LocatedSpan<&'a str>;
pub type ParserError<'a> = SyntaxError<Span<'a>>;
pub type ParseResult<'a, T> = IResult<Span<'a>, T, ParserError<'a>>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SyntaxError<T> {
  input: T,
  kind: ErrorKind,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
  UnexpectedToken,
  Unterminated,
  UnknownType,
  InvalidFileDescriptor,
}

impl<I> SyntaxError<I> {
  pub fn new(input: I, kind: ErrorKind) -> Self {
    SyntaxError { input, kind }
  }
}

impl<I> nom::error::ParseError<I> for SyntaxError<I> {
  fn from_error_kind(input: I, _: nom::error::ErrorKind) -> Self {
    SyntaxError::new(input, ErrorKind::UnexpectedToken)
  }

  fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
    other
  }
}

#[macro_export]
macro_rules! syntax_check {
  ($input:expr, $fn:expr, $kind:expr) => {
    nom::combinator::cut($fn)($input).map_err(|e: nom::Err<super::ParserError>| match e {
      nom::Err::Failure(_) => {
        let e = super::SyntaxError::new($input, $kind);
        nom::Err::Failure(e)
      }
      _ => e,
    })
  };
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Annotate<T> {
  value: T,
  column: usize,
  line: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Space,
  Command(Vec<Vec<Word>>, Vec<Redirect>),
}

pub fn parse_command_line(i: Span) -> Result<Vec<Token>, ParserError> {
  let mut parser = separated_list0(space1, parse_command);
  match parser(i).finish() {
    Ok((_, t)) => Ok(t),
    Err(e) => {
      let line = e.input.location_line();
      let offset = e.input.location_offset();
      let err_line = String::from_utf8(e.input.get_line_beginning().to_vec()).unwrap();
      let code = e.kind;
      eprintln!("error({}): {:?}", line, code);
      eprintln!("{}", err_line);

      let padding = " ".to_string().repeat(offset);
      let hyright_len = e.input.to_string().len();
      let hyright = "^".to_string().repeat(hyright_len);
      eprintln!("{}{}", padding, hyright);
      Ok(vec![])
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
enum CommandFragment {
  Word(Vec<Word>),
  Redirect(Redirect),
}

fn parse_command(i: Span) -> ParseResult<Token> {
  let mut words: Vec<Vec<Word>> = vec![];
  let mut redirect: Vec<Redirect> = vec![];

  let (o, f) = many1(alt((
    map(parse_redirect, CommandFragment::Redirect),
    map(parse_word, CommandFragment::Word),
  )))(i)?;

  f.iter().for_each(|fragment| match fragment {
    CommandFragment::Word(w) => words.push(w.to_vec()),
    CommandFragment::Redirect(r) => redirect.push(r.clone()),
  });

  let command = Token::Command(words, redirect);
  Ok((o, command))
}

#[cfg(test)]
mod test {
  //use super::*;
  use anyhow::Result;

  #[test]
  fn test_parse_command() -> Result<()> {
    //let (_, command) = parse_command(Span::new("echo foo bar"))?;
    //if let Token::Command(cmd, args, redirect) = command {
    //  assert_eq!("echo", cmd.value);
    //  let mut got = args.iter();
    //  assert_eq!("foo", got.next().unwrap().value);
    //  assert_eq!("bar", got.next().unwrap().value);
    //  assert_eq!(0, redirect.len());
    //} else {
    //  unreachable![]
    //}

    Ok(())
  }
}
