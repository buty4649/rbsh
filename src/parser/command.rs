use super::redirect::parse_redirect;
use super::word::parse_wordlist;
use super::{ParseError, TokenKind, UnitKind};
use crate::token::TokenReader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnecterKind {
    Pipe,
    PipeBoth,
    And,
    Or,
}

pub fn parse_command(tokens: &mut TokenReader) -> Result<Option<UnitKind>, ParseError> {
    match parse_connecter(tokens)? {
        None => Ok(None),
        Some(c) => match tokens.peek_token() {
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
                    }
                    | UnitKind::If {
                        condition: _,
                        true_case: _,
                        false_case: _,
                        redirect: _,
                        background,
                    }
                    | UnitKind::Unless {
                        condition: _,
                        false_case: _,
                        true_case: _,
                        redirect: _,
                        background,
                    }
                    | UnitKind::While {
                        condition: _,
                        command: _,
                        redirect: _,
                        background,
                    }
                    | UnitKind::Until {
                        condition: _,
                        command: _,
                        redirect: _,
                        background,
                    } => *background = true,
                };
                Ok(Some(c))
            }
            _ => Ok(Some(c)),
        },
    }
}

pub fn parse_connecter(tokens: &mut TokenReader) -> Result<Option<UnitKind>, ParseError> {
    match tokens.peek_token() {
        None => Ok(None), // EOF
        Some(TokenKind::Space) => {
            tokens.next();
            parse_connecter(tokens)
        }
        Some(_) => {
            match parse_shell_command(tokens)? {
                None => Err(tokens.error_unexpected_token()),
                Some(left) => {
                    tokens.skip_space();
                    match tokens.peek_token() {
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

                        Some(TokenKind::Termination) | Some(TokenKind::NewLine) => {
                            tokens.next();
                            Ok(Some(left))
                        }

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

pub fn parse_shell_command(tokens: &mut TokenReader) -> Result<Option<UnitKind>, ParseError> {
    let mut unit = match tokens.peek_token() {
        Some(TokenKind::If) => parse_if_statement(tokens)?,
        Some(TokenKind::Unless) => parse_unless_statement(tokens)?,
        Some(TokenKind::While) | Some(TokenKind::Until) => parse_while_or_until_statement(tokens)?,
        _ => return parse_simple_command(tokens),
    };

    // parse redirection list
    let unit = match &mut unit {
        Some(UnitKind::If {
            condition: _,
            true_case: _,
            false_case: _,
            redirect,
            background: _,
        })
        | Some(UnitKind::Unless {
            condition: _,
            false_case: _,
            true_case: _,
            redirect,
            background: _,
        })
        | Some(UnitKind::While {
            condition: _,
            command: _,
            redirect,
            background: _,
        })
        | Some(UnitKind::Until {
            condition: _,
            command: _,
            redirect,
            background: _,
        }) => {
            tokens.skip_space();
            loop {
                match parse_redirect(tokens)? {
                    Some(r) => redirect.push(r),
                    None => break,
                }
            }
            unit
        }

        _ => unit,
    };
    Ok(unit)
}

fn parse_if_statement(tokens: &mut TokenReader) -> Result<Option<UnitKind>, ParseError> {
    tokens.next(); // 'if'

    // need space
    match tokens.skip_space() {
        Some(_) => (),
        None => return Err(tokens.error_unexpected_token()),
    };

    let condition = match parse_command(tokens)? {
        Some(c) => c,
        None => return Err(tokens.error_unexpected_token()),
    };

    tokens.skip_space();
    if matches!(tokens.peek_token(), Some(TokenKind::Then)) {
        tokens.next();
    }

    let mut true_case = vec![];
    let mut false_case: Option<_> = None;
    loop {
        match parse_command(tokens)? {
            Some(c) => true_case.push(c),
            None => return Err(tokens.error_eof()),
        };

        tokens.skip_space();
        match tokens.peek_token() {
            Some(TokenKind::Fi) | Some(TokenKind::End) => {
                tokens.next();
                break;
            }
            Some(TokenKind::Else) => {
                tokens.next();
                false_case = parse_else_clause(tokens)?;
                break;
            }
            Some(TokenKind::ElsIf) | Some(TokenKind::ElIf) => {
                false_case = parse_if_statement(tokens).map(|c| Some(vec![c.unwrap()]))?;
                break;
            }
            None => return Err(tokens.error_eof()),
            _ => (),
        };
    }
    let unit = UnitKind::If {
        condition: Box::new(condition),
        true_case,
        false_case,
        redirect: vec![],
        background: false,
    };

    Ok(Some(unit))
}

fn parse_unless_statement(tokens: &mut TokenReader) -> Result<Option<UnitKind>, ParseError> {
    tokens.next(); // 'unless'

    // need space
    match tokens.skip_space() {
        Some(_) => (),
        None => return Err(tokens.error_unexpected_token()),
    };

    let condition = match parse_command(tokens)? {
        Some(c) => c,
        None => return Err(tokens.error_unexpected_token()),
    };

    tokens.skip_space();
    if matches!(tokens.peek_token(), Some(TokenKind::Then)) {
        tokens.next();
    }

    let mut false_case = vec![];
    let mut true_case: Option<_> = None;
    loop {
        match parse_command(tokens)? {
            Some(c) => false_case.push(c),
            None => return Err(tokens.error_eof()),
        };

        tokens.skip_space();
        match tokens.peek_token() {
            Some(TokenKind::End) => {
                tokens.next();
                break;
            }
            Some(TokenKind::Else) => {
                tokens.next();
                true_case = parse_else_clause(tokens)?;
                break;
            }
            None => return Err(tokens.error_eof()),
            _ => (),
        };
    }
    let unit = UnitKind::Unless {
        condition: Box::new(condition),
        false_case,
        true_case,
        redirect: vec![],
        background: false,
    };

    Ok(Some(unit))
}

fn parse_else_clause(tokens: &mut TokenReader) -> Result<Option<Vec<UnitKind>>, ParseError> {
    let mut units = vec![];
    loop {
        match parse_command(tokens)? {
            Some(c) => units.push(c),
            None => return Err(tokens.error_eof()),
        };
        tokens.skip_space();
        match tokens.peek_token() {
            Some(TokenKind::Fi) | Some(TokenKind::End) => {
                tokens.next();
                break Ok(Some(units));
            }
            None => return Err(tokens.error_eof()),
            _ => (),
        }
    }
}

fn parse_while_or_until_statement(
    tokens: &mut TokenReader,
) -> Result<Option<UnitKind>, ParseError> {
    let token = tokens.next().unwrap().value; // 'while' or 'until'

    // need space
    match tokens.skip_space() {
        Some(_) => (),
        None => return Err(tokens.error_unexpected_token()),
    };

    let condition = match parse_command(tokens)? {
        Some(c) => c,
        None => return Err(tokens.error_unexpected_token()),
    };

    tokens.skip_space();
    if matches!(tokens.peek_token(), Some(TokenKind::Do)) {
        tokens.next();
    }

    let mut command = vec![];
    loop {
        match parse_command(tokens)? {
            Some(c) => command.push(c),
            None => return Err(tokens.error_eof()),
        };

        tokens.skip_space();
        match tokens.peek_token() {
            Some(TokenKind::Done) | Some(TokenKind::End) => {
                tokens.next();
                break;
            }
            None => return Err(tokens.error_eof()),
            _ => (),
        };
    }

    let unit = match token {
        TokenKind::While => UnitKind::While {
            condition: Box::new(condition),
            command,
            redirect: vec![],
            background: false,
        },
        TokenKind::Until => UnitKind::Until {
            condition: Box::new(condition),
            command,
            redirect: vec![],
            background: false,
        },
        _ => unimplemented![],
    };

    Ok(Some(unit))
}

fn parse_simple_command(tokens: &mut TokenReader) -> Result<Option<UnitKind>, ParseError> {
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

        match tokens.peek_token() {
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
