pub mod ast;
mod parser;
pub mod string;

use ast::Node;
use parser::rbsh::statement;
use peg::str::LineCol;

pub fn parse(input: &str, rubyish: bool) -> Result<Vec<Node>, peg::error::ParseError<LineCol>> {
    if cfg!(feature = "trace") {
        println!("[PEG_INPUT_START]\n{input}\n[PEG_TRACE_START]");
    }

    let result = statement(input, rubyish);

    if cfg!(feature = "trace") {
        println!("[PEG_TRACE_STOP]");
    }

    result
}
