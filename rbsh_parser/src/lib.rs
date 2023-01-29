pub mod ast;
mod parser;
pub mod string;

use ast::*;

use self::parser::reddish_parser::statement;
use peg::str::LineCol;

pub fn parse(input: &str, rubyish: bool) -> Result<Vec<Node>, peg::error::ParseError<LineCol>> {
    statement(input, rubyish)
}
