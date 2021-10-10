use super::redirect::parse_redirect;
use super::word::parse_wordlist;
use super::{peek_token, ParseError, Token, TokenKind, UnitKind};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnecterKind {
    Pipe,
    PipeBoth,
    And,
    Or,
}

pub fn parse_command<T>(tokens: &mut Peekable<T>) -> Result<Option<UnitKind>, ParseError>
where
    T: Iterator<Item = Token> + Clone,
{
    match parse_connecter(tokens)? {
        None => Ok(None),
        Some(c) => match peek_token(tokens) {
            Some(TokenKind::Background) => {
                tokens.next();
                let mut c = c;
                match &mut c {
                    UnitKind::SimpleCommand {
                        command: _,
                        redirect: _,
                        background,
                    }
                    | UnitKind::Connecter {
                        left: _,
                        right: _,
                        kind: _,
                        background,
                    } => *background = true,
                };
                Ok(Some(c))
            }
            _ => Ok(Some(c)),
        },
    }
}

pub fn parse_connecter<T>(tokens: &mut Peekable<T>) -> Result<Option<UnitKind>, ParseError>
where
    T: Iterator<Item = Token> + Clone,
{
    match peek_token(tokens) {
        None => Ok(None), // EOF
        Some(TokenKind::Space) => {
            tokens.next();
            parse_connecter(tokens)
        }
        Some(_) => {
            match parse_shell_command(tokens)? {
                None => {
                    let token = tokens.next().unwrap();
                    Err(ParseError::unexpected_token(token))
                }
                Some(left) => {
                    match peek_token(tokens) {
                        // Connecter
                        Some(kind)
                            if matches!(
                                kind,
                                TokenKind::Pipe
                                    | TokenKind::PipeBoth
                                    | TokenKind::And
                                    | TokenKind::Or
                            ) =>
                        {
                            let token = tokens.next().unwrap();
                            let kind = match token.value {
                                TokenKind::Pipe => ConnecterKind::Pipe,
                                TokenKind::PipeBoth => ConnecterKind::PipeBoth,
                                TokenKind::And => ConnecterKind::And,
                                TokenKind::Or => ConnecterKind::Or,
                                _ => unreachable![],
                            };
                            let left = Box::new(left);
                            let right = match parse_connecter(tokens)? {
                                Some(c) => Box::new(c),
                                None => return Err(ParseError::unexpected_token(token)),
                            };
                            let connecter = UnitKind::Connecter {
                                left,
                                right,
                                kind,
                                background: false,
                            };
                            Ok(Some(connecter))
                        }

                        Some(kind)
                            if matches!(kind, TokenKind::Termination | TokenKind::NewLine) =>
                        {
                            tokens.next();
                            Ok(Some(left))
                        }

                        // Do not nothing.
                        // Set the background flag in the caller.
                        Some(TokenKind::Background) => Ok(Some(left)),

                        // None or Some(_)
                        _ => Ok(Some(left)),
                    }
                }
            }
        }
    }
}

pub fn parse_shell_command<T>(tokens: &mut Peekable<T>) -> Result<Option<UnitKind>, ParseError>
where
    T: Iterator<Item = Token> + Clone,
{
    parse_simple_command(tokens)
}

fn parse_simple_command<T>(tokens: &mut Peekable<T>) -> Result<Option<UnitKind>, ParseError>
where
    T: Iterator<Item = Token> + Clone,
{
    let mut command = vec![];
    let mut redirect = vec![];

    loop {
        match parse_redirect(tokens)? {
            None => (),
            Some(r) => {
                redirect.push(r);
                continue;
            }
        }

        match peek_token(tokens) {
            Some(TokenKind::Space) => {
                tokens.next();
            }
            Some(TokenKind::Word(_, _)) => {
                let words = parse_wordlist(tokens)?;
                command.push(words);
            }
            _ => break,
        }
    }

    if command.is_empty() && redirect.is_empty() {
        Ok(None)
    } else {
        Ok(Some(UnitKind::SimpleCommand {
            command,
            redirect,
            background: false,
        }))
    }
}

include!("command_test.rs");
