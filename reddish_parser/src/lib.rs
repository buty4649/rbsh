mod error;
mod location;
mod parser;

pub use error::{Error, ErrorKind};
pub use location::{Annotate, Location};
pub use parser::{
    parse_command_line, CommandList, ConnecterKind, Redirect, RedirectKind, RedirectList, Token,
    TokenKind, Unit, UnitKind, Word, WordKind, WordList,
};

pub type Result<T> = std::result::Result<T, Error>;
