mod debug;
mod redirect;
mod unit;

pub use redirect::{parse_redirect, Redirect, RedirectKind};
pub use unit::{ConnecterKind, Unit, UnitKind};

use crate::{
    lexer::{Lexer, LexerIterator},
    Error, Result, TokenKind, Word,
};

pub fn parse_command_line<S: AsRef<str>>(input: S, offset: usize) -> Result<(Vec<Unit>, bool)> {
    let mut lexer = Lexer::new(input.as_ref(), offset).iter();

    // If it starts with a Space, ignore the command history.
    let ignore_history = lexer.skip_if_space()?;

    let mut result = Vec::new();
    while let Some(unit) = parse_command(&mut lexer)? {
        result.push(unit)
    }

    debug::print(&result);
    Ok((result, ignore_history))
}

fn parse_command(lexer: &mut LexerIterator) -> Result<Option<Unit>> {
    lexer.skip_if_space()?;
    parse_newline_or_termination(lexer)?;

    match lexer.peek() {
        None => Ok(None),
        Some(_) => match parse_connecter(lexer)? {
            None => Err(error_unexpected_token(lexer)),
            Some(kind) => {
                let background = match lexer.next_if(|kind| kind == &TokenKind::Background) {
                    Some(Ok(_)) => true,
                    Some(Err(e)) => return Err(e),
                    None => false,
                };
                Ok(Some(Unit::new(kind, background)))
            }
        },
    }
}

fn parse_connecter(lexer: &mut LexerIterator) -> Result<Option<UnitKind>> {
    match parse_shell_command(lexer)? {
        None => Ok(None),
        Some(left) => {
            lexer.skip_if_space()?;
            match lexer.next_if(|kind| {
                matches!(
                    kind,
                    &TokenKind::Pipe | &TokenKind::PipeBoth | &TokenKind::And | &TokenKind::Or
                )
            }) {
                Some(Ok(token)) => match token.value {
                    TokenKind::Pipe | TokenKind::PipeBoth | TokenKind::And | TokenKind::Or => {
                        let left = Box::new(Unit::new(left, false));
                        let right = parse_command(lexer).and_then(|result| {
                            result.map_or_else(
                                || Err(error_unexpected_token(lexer)),
                                |c| Ok(Box::new(c)),
                            )
                        })?;

                        let unit = match token.value {
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
                            _ => unreachable![],
                        };
                        Ok(Some(unit))
                    }
                    _ => unreachable![],
                },
                Some(Err(e)) => Err(e),
                None => Ok(Some(left)),
            }
        }
    }
}

fn parse_shell_command(lexer: &mut LexerIterator) -> Result<Option<UnitKind>> {
    match lexer.next_if(|kind| {
        matches!(
            kind,
            &TokenKind::If
                | &TokenKind::Unless
                | &TokenKind::While
                | &TokenKind::Until
                | &TokenKind::For
        )
    }) {
        Some(Ok(token)) => {
            need_space(lexer)?;

            let kind = token.value;
            match kind {
                TokenKind::If | TokenKind::Unless => {
                    parse_if_statement(lexer, kind == TokenKind::Unless).map(Some)
                }
                TokenKind::While | TokenKind::Until => {
                    parse_while_statement(lexer, kind == TokenKind::Until).map(Some)
                }
                TokenKind::For => parse_for_statement(lexer).map(Some),
                _ => unreachable![],
            }
        }
        Some(Err(e)) => Err(e),
        None => parse_simple_command(lexer),
    }
}

fn parse_if_statement(lexer: &mut LexerIterator, reverse: bool) -> Result<UnitKind> {
    let condition = Box::new(need_command(lexer)?);
    lexer.skip_if_space()?;
    need_newline_or_termination(lexer)?;
    if let Some(result) = lexer.next_if(|kind| kind == &TokenKind::Then) {
        result?;
    }

    let mut case1 = vec![];
    let mut case2 = None;
    loop {
        lexer.skip_if_space()?;
        parse_newline_or_termination(lexer)?;
        case1.push(need_command(lexer)?);

        lexer.skip_if_space()?;
        parse_newline_or_termination(lexer)?;

        match lexer.next_if(|kind| {
            matches!(
                kind,
                &TokenKind::Fi
                    | &TokenKind::End
                    | &TokenKind::Else
                    | &TokenKind::ElsIf
                    | &TokenKind::ElIf
            )
        }) {
            Some(Ok(token)) => match token.value {
                // fi/elsif/elif is not allow in unless
                TokenKind::Fi | TokenKind::ElsIf | TokenKind::ElIf if reverse => {
                    return Err(Error::unexpected_token(&token))
                }
                TokenKind::Fi | TokenKind::End => break,
                TokenKind::Else => {
                    case2 = Some(parse_else_clause(lexer)?);
                    break;
                }
                TokenKind::ElsIf | TokenKind::ElIf => {
                    case2 = Some(vec![Unit::new(parse_if_statement(lexer, false)?, false)]);
                    break;
                }
                _ => unreachable![],
            },
            Some(Err(e)) => return Err(e),
            None if lexer.peek().is_none() => return Err(Error::eof(lexer.location())),
            None => (),
        }
    }

    lexer.skip_if_space()?;
    let redirect = parse_redirect(lexer)?;

    Ok(if reverse {
        UnitKind::Unless {
            condition,
            false_case: case1,
            true_case: case2,
            redirect,
        }
    } else {
        UnitKind::If {
            condition,
            true_case: case1,
            false_case: case2,
            redirect,
        }
    })
}

fn parse_else_clause(lexer: &mut LexerIterator) -> Result<Vec<Unit>> {
    let mut units = Vec::new();
    loop {
        lexer.skip_if_space()?;
        parse_newline_or_termination(lexer)?;
        units.push(need_command(lexer)?);

        lexer.skip_if_space()?;
        parse_newline_or_termination(lexer)?;
        match lexer.next_if(|kind| matches!(kind, &TokenKind::Fi | &TokenKind::End)) {
            Some(Ok(_)) => break Ok(units),
            Some(Err(e)) => break Err(e),
            None if lexer.peek().is_none() => break Err(Error::eof(lexer.location())),
            None => (), // next
        }
    }
}

fn parse_while_statement(lexer: &mut LexerIterator, reverse: bool) -> Result<UnitKind> {
    let condition = Box::new(need_command(lexer)?);
    lexer.skip_if_space()?;
    need_newline_or_termination(lexer)?;
    if let Some(result) = lexer.next_if(|kind| kind == &TokenKind::Do) {
        result?;
    }

    let mut command = Vec::new();
    loop {
        lexer.skip_if_space()?;
        parse_newline_or_termination(lexer)?;
        command.push(need_command(lexer)?);

        lexer.skip_if_space()?;
        parse_newline_or_termination(lexer)?;
        match lexer.next_if(|kind| matches!(kind, &TokenKind::Done | &TokenKind::End)) {
            Some(Ok(_)) => break,
            Some(Err(e)) => return Err(e),
            None if lexer.peek().is_none() => return Err(Error::eof(lexer.location())),
            None => (), // next
        }
    }

    lexer.skip_if_space()?;
    let redirect = parse_redirect(lexer)?;

    Ok(if reverse {
        UnitKind::Until {
            condition,
            command,
            redirect,
        }
    } else {
        UnitKind::While {
            condition,
            command,
            redirect,
        }
    })
}

fn parse_for_statement(lexer: &mut LexerIterator) -> Result<UnitKind> {
    let identifier = parse_wordlist(lexer).and_then(|result| match result {
        Some(wordlist) => Ok(wordlist),
        None => Err(error_unexpected_token(lexer)),
    })?;

    lexer.skip_if_space_or_newline()?;
    let list =
        lexer
            .next_if(|kind| kind == &TokenKind::In)
            .map_or(Ok(None), |result| match result {
                Ok(_) => {
                    need_space(lexer)?;
                    let mut list = Vec::new();
                    while let Some(wordlist) = parse_wordlist(lexer)? {
                        list.push(wordlist);
                        lexer.skip_if_space()?;
                    }
                    Ok(Some(list))
                }
                Err(e) => Err(e),
            })?;

    lexer.skip_if_space()?;
    parse_newline_or_termination(lexer)?;
    if let Some(result) = lexer.next_if(|kind| kind == &TokenKind::Do) {
        result?;
    }

    let mut command = Vec::new();
    loop {
        lexer.skip_if_space()?;
        parse_newline_or_termination(lexer)?;
        command.push(need_command(lexer)?);

        lexer.skip_if_space()?;
        parse_newline_or_termination(lexer)?;
        match lexer.next_if(|kind| matches!(kind, &TokenKind::Done | &TokenKind::End)) {
            Some(Ok(_)) => break,
            Some(Err(e)) => return Err(e),
            None if lexer.peek().is_none() => return Err(Error::eof(lexer.location())),
            None => (),
        }
    }

    lexer.skip_if_space()?;
    let redirect = parse_redirect(lexer)?;

    Ok(UnitKind::For {
        identifier,
        list,
        command,
        redirect,
    })
}

fn parse_simple_command(lexer: &mut LexerIterator) -> Result<Option<UnitKind>> {
    let mut command = Vec::new();
    let mut redirect = None;

    loop {
        lexer.skip_if_space()?;
        if let Some(mut r) = parse_redirect(lexer)? {
            redirect.get_or_insert(Vec::new()).append(&mut r)
        }

        lexer.skip_if_space()?;
        match parse_wordlist(lexer)? {
            Some(wordlist) => command.push(wordlist),
            None => break,
        }
    }

    if command.is_empty() && redirect.is_none() {
        Ok(None)
    } else {
        Ok(Some(UnitKind::SimpleCommand { command, redirect }))
    }
}

fn parse_wordlist(lexer: &mut LexerIterator) -> Result<Option<Vec<Word>>> {
    let mut result = Vec::new();

    while let Some(token) = lexer.next_if(|kind| matches!(kind, TokenKind::Word { .. })) {
        let token = token?;
        let kind = token.value;
        let location = token.location;
        match kind {
            TokenKind::Word(s, k) => result.push(Word::new(s, k, location)),
            _ => unreachable![],
        }
    }

    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

fn parse_newline_or_termination(lexer: &mut LexerIterator) -> Result<Option<()>> {
    match lexer.next_if(|kind| matches!(kind, &TokenKind::NewLine | &TokenKind::Termination)) {
        Some(Ok(_)) => {
            while let Some(result) = lexer.next_if(|kind| {
                matches!(
                    kind,
                    &TokenKind::Space | &TokenKind::NewLine | &TokenKind::Termination
                )
            }) {
                result?;
            }
            Ok(Some(()))
        }
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

fn need_command(lexer: &mut LexerIterator) -> Result<Unit> {
    parse_command(lexer).and_then(|result| result.ok_or_else(|| error_unexpected_token(lexer)))
}

fn need_space(lexer: &mut LexerIterator) -> Result<()> {
    if lexer.skip_if_space()? {
        Ok(())
    } else {
        Err(error_unexpected_token(lexer))
    }
}

fn need_newline_or_termination(lexer: &mut LexerIterator) -> Result<()> {
    parse_newline_or_termination(lexer)
        .and_then(|result| result.ok_or_else(|| error_unexpected_token(lexer)))
}

fn error_unexpected_token(lexer: &mut LexerIterator) -> Error {
    match lexer.next() {
        Some(Ok(token)) => Error::unexpected_token(&token),
        Some(Err(e)) => e,
        None => Error::eof(lexer.location()),
    }
}

include!("tests.rs");
