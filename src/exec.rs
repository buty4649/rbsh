mod redirect;
mod syscall;

pub use syscall::SysCallError;

use super::{
    context::Context,
    error::{ShellError, ShellErrorKind},
    location::Location,
    parser::{
        redirect::RedirectList,
        word::{Word, WordKind, WordList},
        CommandList, ConnecterKind, UnitKind,
    },
    status::{ExitStatus, Result},
};
use nix::{errno::Errno, sys::wait::WaitStatus, unistd::ForkResult};
use redirect::ApplyRedirect;
use syscall::{SysCallWrapper, Wrapper};

use is_executable::IsExecutable;
use std::{collections::HashMap, env, ffi::CString, path::PathBuf};

pub trait WordParser {
    fn to_string<'a>(self, context: &Context) -> String;
}

impl WordParser for Word {
    fn to_string(self, context: &Context) -> String {
        let (s, k, _) = self.take();
        match k {
            WordKind::Normal | WordKind::Quote | WordKind::Literal => s,
            WordKind::Command => "".to_string(), // unimplemented
            WordKind::Variable | WordKind::Parameter => {
                context.get_var(s).unwrap_or("".to_string())
            }
        }
    }
}

impl WordParser for WordList {
    fn to_string(self, context: &Context) -> String {
        self.to_vec()
            .into_iter()
            .fold(String::new(), |mut result, word| {
                result.push_str(&*word.to_string(context));
                result
            })
    }
}

pub struct Executor<'a> {
    list: Vec<UnitKind>,
    pos: usize,
    context: &'a Context,
    wrapper: Wrapper,
}

pub trait IsPresent {
    fn is_present(&self) -> bool;
}
impl<T> IsPresent for Vec<T> {
    fn is_present(&self) -> bool {
        !self.is_empty()
    }
}
impl<T, U> IsPresent for HashMap<T, U> {
    fn is_present(&self) -> bool {
        !self.is_empty()
    }
}

pub trait ToCString<T> {
    fn to_cstring(self) -> T;
}

impl ToCString<CString> for &str {
    fn to_cstring(self) -> CString {
        CString::new(self).unwrap()
    }
}

impl ToCString<Vec<CString>> for Vec<String> {
    fn to_cstring(self) -> Vec<CString> {
        self.into_iter().map(|s| s.to_cstring()).collect::<Vec<_>>()
    }
}

pub trait IsVarName {
    fn is_var_name(&self) -> bool;
}

impl IsVarName for WordList {
    fn is_var_name(&self) -> bool {
        match self.first().take() {
            (string, WordKind::Normal, _) => {
                let mut c = string.chars();

                // first char is must alphanumeric
                match c.next() {
                    Some(c) if c.is_alphanumeric() => true,
                    _ => return false,
                };

                loop {
                    match c.next() {
                        Some(c) if c == '=' => break true,
                        Some(c) if c.is_alphanumeric() || c == '_' => continue,
                        _ => break false,
                    }
                }
            }
            _ => false,
        }
    }
}

pub trait ShellExecute {
    fn execute(&self, context: &Context) -> Result<ExitStatus>;
}

impl ShellExecute for Vec<UnitKind> {
    fn execute(&self, context: &Context) -> Result<ExitStatus> {
        Executor::new(self.to_vec(), context).execute()
    }
}

impl ShellExecute for CommandList {
    fn execute(&self, context: &Context) -> Result<ExitStatus> {
        self.to_vec().execute(context)
    }
}

impl<'a> Executor<'a> {
    pub fn new(list: Vec<UnitKind>, context: &'a Context) -> Self {
        Self {
            list,
            pos: 0,
            context,
            wrapper: Wrapper::new(),
        }
    }

    #[cfg(test)]
    fn set_wrapper(&mut self, wrapper: Wrapper) {
        self.wrapper = wrapper
    }

    fn next(&mut self) -> Option<UnitKind> {
        if self.pos >= self.list.len() {
            None
        } else {
            self.pos += 1;
            Some(self.list[self.pos - 1].clone())
        }
    }

    pub fn execute(&mut self) -> Result<ExitStatus> {
        let mut status = ExitStatus::new(0); // noop
        loop {
            match self.next() {
                Some(c) => match self.execute_command(c) {
                    Ok(s) => status = s,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return Err(e);
                    }
                },
                None => break, // noop
            };
        }
        Ok(status)
    }

    fn execute_command(&self, cmd: UnitKind) -> Result<ExitStatus> {
        match cmd {
            UnitKind::SimpleCommand {
                command,
                redirect,
                background,
            } => self.execute_simple_command(command, redirect, background),
            UnitKind::Connecter {
                left,
                right,
                kind,
                background,
            } => self.execute_connecter(left, right, kind, background),
            UnitKind::If {
                condition,
                true_case,
                false_case,
                redirect,
                background,
            } => self.execute_if_command(
                condition, true_case, false_case, redirect, background, false,
            ),
            UnitKind::Unless {
                condition,
                false_case,
                true_case,
                redirect,
                background,
            } => self
                .execute_if_command(condition, false_case, true_case, redirect, background, true),
            UnitKind::While {
                condition,
                command,
                redirect,
                background,
            } => self.execute_while_command(condition, command, redirect, background, false),
            UnitKind::Until {
                condition,
                command,
                redirect,
                background,
            } => self.execute_while_command(condition, command, redirect, background, true),
            UnitKind::For {
                identifier,
                list,
                command,
                redirect,
                background,
            } => self.execute_for_command(identifier, list, command, redirect, background),
        }
    }

    fn execute_simple_command(
        &self,
        command: Vec<WordList>,
        redirect: RedirectList,
        _background: bool,
    ) -> Result<ExitStatus> {
        let (temp_env, cmds) = split_env_and_commands(command, self.context);
        if cmds.is_empty() && temp_env.is_present() {
            temp_env
                .iter()
                .for_each(|(k, v)| self.context.set_var(k, v));
            Ok(ExitStatus::new(0))
        } else if cmds.is_present() {
            match self.wrapper.fork() {
                Err(e) => Err(ShellError::syscall_error(e, Location::new(1, 1))),
                Ok(ForkResult::Parent { child }) => match self.wrapper.waitpid(child, None) {
                    Ok(WaitStatus::Exited(_, status)) => Ok(ExitStatus::new(status)),
                    Err(e) => Err(ShellError::syscall_error(e, Location::new(1, 1))),
                    _ => unimplemented![],
                },
                Ok(ForkResult::Child) => {
                    let cmdpath = cmds.first().unwrap().to_string();
                    let status = self.execute_external_command(&*cmdpath, cmds, temp_env, redirect);
                    Ok(status) // Only reachable with cfg(test).
                }
            }
        } else {
            // noop
            Ok(ExitStatus::new(0))
        }
    }

    fn execute_external_command(
        &self,
        path: &str,
        cmds: Vec<String>,
        temp_env: HashMap<String, String>,
        redirect: RedirectList,
    ) -> ExitStatus {
        let cmdpath = assume_command(path).to_str().unwrap().to_cstring();
        let cmds = cmds.to_cstring();

        // merge temporary env to env
        let mut env = env::vars().collect::<HashMap<_, _>>();
        temp_env.into_iter().for_each(|(k, v)| {
            env.insert(k, v);
        });
        let env = env
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v).to_cstring())
            .collect::<Vec<_>>();

        if let Err(e) = redirect.apply(self.context) {
            match e.value() {
                ShellErrorKind::SysCallError(f, e) => {
                    eprintln!("{}: {}", f, e.desc());
                    return self.wrapper.exit(1);
                }
                _ => unreachable![],
            }
        }

        match self.wrapper.execve(cmdpath, &cmds, &env) {
            Ok(_) => unreachable![],
            Err(e) if e.errno() == Errno::ENOENT => {
                eprintln!("{}: command not found", path);
                self.wrapper.exit(127)
            }
            Err(e) => {
                eprintln!("execve faile: {}", e.errno());
                self.wrapper.exit(1)
            }
        }
    }

    fn execute_connecter(
        &self,
        left: Box<UnitKind>,
        right: Box<UnitKind>,
        kind: ConnecterKind,
        _background: bool,
    ) -> Result<ExitStatus> {
        match kind {
            ConnecterKind::And => match vec![*left].execute(self.context)? {
                st if st.is_error() => Ok(st),
                _ => vec![*right].execute(self.context),
            },
            ConnecterKind::Or => match vec![*left].execute(self.context)? {
                st if st.is_success() => Ok(st),
                _ => vec![*right].execute(self.context),
            },
            ConnecterKind::Pipe => unimplemented![],
            ConnecterKind::PipeBoth => unimplemented![],
        }
    }

    fn execute_if_command(
        &self,
        condition: Box<UnitKind>,
        true_case: Vec<UnitKind>,
        false_case: Option<Vec<UnitKind>>,
        _redirect: RedirectList,
        _background: bool,
        inverse: bool,
    ) -> Result<ExitStatus> {
        match self.execute_command(*condition)? {
            status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                true_case.execute(self.context)
            }
            status if false_case.is_none() => Ok(status),
            _ => false_case.unwrap().execute(self.context),
        }
    }

    fn execute_while_command(
        &self,
        condition: Box<UnitKind>,
        command: Vec<UnitKind>,
        _redirect: RedirectList,
        _background: bool,
        inverse: bool,
    ) -> Result<ExitStatus> {
        loop {
            match self.execute_command(*condition.clone())? {
                status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                    command.execute(self.context)?;
                }
                _ => break,
            }
        }
        Ok(ExitStatus::new(0))
    }

    fn execute_for_command(
        &self,
        identifier: Word,
        list: Option<Vec<WordList>>,
        command: Vec<UnitKind>,
        _redirect: RedirectList,
        _background: bool,
    ) -> Result<ExitStatus> {
        let identifier = match identifier.take() {
            (string, kind, _) if kind == WordKind::Normal => string.to_string(),
            (string, kind, loc) => {
                let invalid_identifier = match kind {
                    WordKind::Normal => unreachable![],
                    WordKind::Quote => format!("\"{}\"", string),
                    WordKind::Literal => format!("'{}'", string),
                    WordKind::Command => format!("`{}`", string),
                    WordKind::Variable => format!("${}", string),
                    WordKind::Parameter => format!("${{{}}}", string),
                };
                return Err(ShellError::invalid_identifier(invalid_identifier, loc));
            }
        };

        let list = match list {
            None => vec![], // Normally, it returns $0.
            Some(list) => list
                .into_iter()
                .map(|w| w.to_string(self.context))
                .collect::<Vec<_>>(),
        };

        for word in list.iter() {
            let context = self.context.clone();
            context.set_var(&*identifier, word);
            command.execute(&context)?;
        }

        Ok(ExitStatus::new(0))
    }
}

fn split_env_and_commands(
    list: Vec<WordList>,
    context: &Context,
) -> (HashMap<String, String>, Vec<String>) {
    let (env, cmds) = {
        let mut env = vec![];
        let mut cmds = vec![];
        let mut iter = list.into_iter().peekable();

        loop {
            match iter.peek() {
                Some(wl) if wl.is_var_name() => {
                    let wl = iter.next().unwrap();
                    env.push(wl.clone())
                }
                _ => break,
            }
        }

        loop {
            match iter.next() {
                Some(wl) => cmds.push(wl.clone()),
                None => break,
            }
        }

        (env, cmds)
    };

    let env = env
        .into_iter()
        .map(|wordlist| {
            wordlist
                .to_string(context)
                .split_once("=")
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .unwrap()
        })
        .collect::<HashMap<_, _>>();
    let cmds = cmds
        .into_iter()
        .map(|wordlist| wordlist.to_string(context))
        .collect::<Vec<_>>();
    (env, cmds)
}

fn assume_command(command: &str) -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(command);

    if buf.is_absolute() || buf.starts_with(".") {
        buf
    } else {
        // search command
        match env::var("PATH") {
            Err(_) => buf,
            Ok(val) => val
                .split(":")
                .find_map(|p| {
                    let mut buf = PathBuf::new();
                    buf.push(p);
                    buf.push(command);
                    if buf.is_file() && buf.is_executable() {
                        Some(buf)
                    } else {
                        None
                    }
                })
                .unwrap_or(buf),
        }
    }
}

include!("exec_test.rs");
