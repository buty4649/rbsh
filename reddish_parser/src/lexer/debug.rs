use super::Token;

#[cfg(feature = "debug")]
macro_rules! debug {
    ($($args:tt)*) => {{
       eprint!("debug(lexer): ");
       eprintln!($($args)*);
    }};
}

#[cfg(not(feature = "debug"))]
pub(crate) fn print(_: &[Token]) {}

#[cfg(feature = "debug")]
pub(crate) fn print(tokens: &[Token]) {
    debug!("results:");
    tokens.iter().for_each(|t| debug!("  {:?}", t));
    debug!("");
}
