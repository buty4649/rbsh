use super::{
    redirect::parse_redirect,
    token::TokenReader,
    word::{parse_wordlist, Word},
    {TokenKind, Unit, UnitKind},
};
use crate::{error::ShellError, status::Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnecterKind {
    And,
    Or,
}

pub fn parse_command(tokens: &mut TokenReader) -> Result<Option<Unit>> {
    match parse_connecter(tokens)? {
        None => Ok(None),
        Some(kind) => {
            let background = match tokens.peek_token() {
                Some(TokenKind::Background) => {
                    tokens.next();
                    true
                }
                _ => false,
            };
            Ok(Some(Unit::new(kind, background)))
        }
    }
}

pub fn parse_connecter(tokens: &mut TokenReader) -> Result<Option<UnitKind>> {
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
                    tokens.skip_space(false);
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
                            let left = Box::new(Unit::new(left, false));
                            let right = match parse_connecter(tokens)? {
                                Some(c) => Box::new(Unit::new(c, false)),
                                None => return Err(ShellError::unexpected_token(token)),
                            };

                            let unit = match token.value() {
                                TokenKind::And => UnitKind::Connecter {
                                    left,
                                    right,
                                    kind: ConnecterKind::And,
                                },
                                TokenKind::Or => UnitKind::Connecter {
                                    left,
                                    right,
                                    kind: ConnecterKind::Or,
                                },
                                TokenKind::Pipe => UnitKind::Pipe {
                                    left,
                                    right,
                                    both: false,
                                },
                                TokenKind::PipeBoth => UnitKind::Pipe {
                                    left,
                                    right,
                                    both: true,
                                },
                                _ => unreachable![],
                            };
                            Ok(Some(unit))
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

pub fn parse_shell_command(tokens: &mut TokenReader) -> Result<Option<UnitKind>> {
    let mut unit = match tokens.peek_token() {
        Some(TokenKind::If) => parse_if_statement(tokens)?,
        Some(TokenKind::Unless) => parse_unless_statement(tokens)?,
        Some(TokenKind::While) | Some(TokenKind::Until) => parse_while_or_until_statement(tokens)?,
        Some(TokenKind::For) => parse_for_statement(tokens)?,
        _ => return parse_simple_command(tokens),
    };

    // parse redirection list
    let unit = match &mut unit {
        Some(UnitKind::If { redirect, .. })
        | Some(UnitKind::Unless { redirect, .. })
        | Some(UnitKind::While { redirect, .. })
        | Some(UnitKind::Until { redirect, .. })
        | Some(UnitKind::For { redirect, .. }) => {
            tokens.skip_space(false);
            while let Some(r) = parse_redirect(tokens)? {
                redirect.push(r)
            }
            unit
        }

        _ => unit,
    };
    Ok(unit)
}

fn parse_if_statement(tokens: &mut TokenReader) -> Result<Option<UnitKind>> {
    tokens.next(); // 'if'

    // need space
    match tokens.skip_space(true) {
        Some(_) => (),
        None => return Err(tokens.error_unexpected_token()),
    };

    let condition = match parse_command(tokens)? {
        Some(c) => c,
        None => return Err(tokens.error_unexpected_token()),
    };

    tokens.skip_space(true);
    if matches!(tokens.peek_token(), Some(TokenKind::Then)) {
        tokens.next();
        tokens.skip_space(true);
    }

    let mut true_case = vec![];
    let mut false_case = None;
    loop {
        match parse_command(tokens)? {
            Some(c) => true_case.push(c),
            None => return Err(tokens.error_eof()),
        };

        tokens.skip_space(true);
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
                false_case =
                    parse_if_statement(tokens).map(|c| Some(vec![Unit::new(c.unwrap(), false)]))?;
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
    };

    Ok(Some(unit))
}

fn parse_unless_statement(tokens: &mut TokenReader) -> Result<Option<UnitKind>> {
    tokens.next(); // 'unless'

    // need space
    match tokens.skip_space(true) {
        Some(_) => (),
        None => return Err(tokens.error_unexpected_token()),
    };

    let condition = match parse_command(tokens)? {
        Some(c) => c,
        None => return Err(tokens.error_unexpected_token()),
    };

    tokens.skip_space(true);
    if matches!(tokens.peek_token(), Some(TokenKind::Then)) {
        tokens.next();
        tokens.skip_space(true);
    }

    let mut false_case = vec![];
    let mut true_case = None;
    loop {
        match parse_command(tokens)? {
            Some(c) => false_case.push(c),
            None => return Err(tokens.error_eof()),
        };

        tokens.skip_space(true);
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
    };

    Ok(Some(unit))
}

fn parse_else_clause(tokens: &mut TokenReader) -> Result<Option<Vec<Unit>>> {
    let mut units = vec![];
    loop {
        match parse_command(tokens)? {
            Some(c) => units.push(c),
            None => return Err(tokens.error_eof()),
        };
        tokens.skip_space(true);
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

fn parse_while_or_until_statement(tokens: &mut TokenReader) -> Result<Option<UnitKind>> {
    let token = tokens.next().unwrap().value(); // 'while' or 'until'

    // need space
    match tokens.skip_space(true) {
        Some(_) => (),
        None => return Err(tokens.error_unexpected_token()),
    };

    let condition = match parse_command(tokens)? {
        Some(c) => c,
        None => return Err(tokens.error_unexpected_token()),
    };

    tokens.skip_space(true);
    if matches!(tokens.peek_token(), Some(TokenKind::Do)) {
        tokens.next();
        tokens.skip_space(true);
    }

    let mut command = vec![];
    loop {
        match parse_command(tokens)? {
            Some(c) => command.push(c),
            None => return Err(tokens.error_eof()),
        };

        tokens.skip_space(true);
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
        },
        TokenKind::Until => UnitKind::Until {
            condition: Box::new(condition),
            command,
            redirect: vec![],
        },
        _ => unimplemented![],
    };

    Ok(Some(unit))
}

fn parse_for_statement(tokens: &mut TokenReader) -> Result<Option<UnitKind>> {
    tokens.next(); // 'for'

    // need space
    match tokens.skip_space(false) {
        Some(_) => (),
        None => return Err(tokens.error_unexpected_token()),
    };

    // need word
    let identifier = match tokens.peek_token() {
        Some(TokenKind::Word(s, kind)) => {
            let loc = tokens.location();
            tokens.next();
            Word::new(s, kind, loc)
        }
        _ => return Err(tokens.error_unexpected_token()),
    };

    tokens.skip_space(false);
    let list = tokens
        .next_if(|k| k == &TokenKind::In)
        .map_or(Ok(None), |_| {
            tokens.skip_space(false);
            let mut wordlist = vec![];
            while let Some(TokenKind::Word(_, _)) = tokens.peek_token() {
                let words = parse_wordlist(tokens)?;
                wordlist.push(words);
            }
            Ok(Some(wordlist))
        })?;

    tokens.skip_space(false);
    match tokens.peek_token() {
        Some(TokenKind::Termination | TokenKind::NewLine) => {
            tokens.next();
        }
        _ => return Err(tokens.error_unexpected_token()),
    }

    tokens.skip_space(true);
    let terminate_group_end = match tokens.peek_token() {
        Some(TokenKind::GroupStart) => {
            tokens.next();
            true
        }
        Some(TokenKind::Do) => {
            tokens.next();
            false
        }
        _ => false,
    };

    let mut command = vec![];
    loop {
        match parse_command(tokens)? {
            Some(c) => command.push(c),
            None => return Err(tokens.error_eof()),
        };

        tokens.skip_space(true);
        match tokens.peek_token() {
            Some(TokenKind::GroupEnd) if terminate_group_end => {
                tokens.next();
                break;
            }
            Some(TokenKind::Done) | Some(TokenKind::End) if !terminate_group_end => {
                tokens.next();
                break;
            }
            None => return Err(tokens.error_eof()),
            _ => (),
        };
    }

    let unit = UnitKind::For {
        identifier,
        list,
        command,
        redirect: vec![],
    };
    Ok(Some(unit))
}

fn parse_simple_command(tokens: &mut TokenReader) -> Result<Option<UnitKind>> {
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
        Ok(Some(UnitKind::SimpleCommand { command, redirect }))
    }
}

include!("command_test.rs");
