mod error;
mod lexer;
mod location;
mod parser;
mod token;
mod word;

pub use error::{Error, ErrorKind};
pub use location::{Annotate, Location};
pub use parser::{parse_command_line, ConnecterKind, Redirect, RedirectKind, Unit, UnitKind};
pub use token::{Token, TokenKind};
pub use word::{Word, WordKind};

pub type Result<T> = std::result::Result<T, Error>;
