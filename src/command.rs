mod redirect;

use crate::{
    error::ShellError,
    parser::{
        redirect::RedirectList,
        word::{Word, WordKind, WordList},
        CommandList, UnitKind,
    },
    ExitStatus, Location, Result,
};
use is_executable::IsExecutable;
use nix::{
    errno::Errno,
    sys::wait::{waitpid, WaitStatus},
    unistd::{execve, fork, ForkResult},
};
use redirect::ApplyRedirect;
use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::path::PathBuf;
use std::process::exit;

pub trait WordParser {
    fn to_string(self) -> String;
}

impl WordParser for Word {
    fn to_string(self) -> String {
        let (s, k, _) = self.take();
        match k {
            WordKind::Normal | WordKind::Quote | WordKind::Literal => s,
            WordKind::Command | WordKind::Variable | WordKind::Parameter => "".to_string(),
        }
    }
}

impl WordParser for WordList {
    fn to_string(self) -> String {
        self.to_vec()
            .into_iter()
            .fold(String::new(), |mut result, word| {
                result.push_str(&*word.to_string());
                result
            })
    }
}

pub struct Executor {
    list: Vec<UnitKind>,
    pos: usize,
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

impl Executor {
    pub fn new(list: Vec<UnitKind>) -> Self {
        Self { list, pos: 0 }
    }

    pub fn new_from(list: CommandList) -> Self {
        Self::new(list.to_vec())
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
            _ => unimplemented![],
        }
    }

    fn execute_simple_command(
        &self,
        command: Vec<WordList>,
        redirect: RedirectList,
        _background: bool,
    ) -> Result<ExitStatus> {
        match unsafe { fork() } {
            Err(e) => Err(ShellError::syscall_error("fork", e, Location::new(1, 1))),
            Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
                Ok(WaitStatus::Exited(_, status)) => Ok(ExitStatus::new(status)),
                Err(e) => Err(ShellError::syscall_error("waitpid", e, Location::new(1, 1))),
                _ => unimplemented![],
            },
            Ok(ForkResult::Child) => {
                let (temp_env, cmds) = split_env_and_commands(command);
                if cmds.is_empty() && temp_env.is_present() {
                    // Env set
                } else if cmds.is_present() {
                    let cmdpath = cmds.first().unwrap().to_string();
                    self.execute_external_command(&*cmdpath, cmds, temp_env, redirect);
                }
                // noop
                exit(0)
            }
        }
    }

    fn execute_external_command(
        &self,
        path: &str,
        cmds: Vec<String>,
        temp_env: HashMap<String, String>,
        redirect: RedirectList,
    ) {
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

        redirect.apply();
        match execve(&cmdpath, &cmds, &env) {
            Ok(_) => unreachable![],
            Err(Errno::ENOENT) => {
                eprintln!("{}: command not found", path);
                exit(127)
            }
            Err(e) => {
                eprintln!("execve faile: {:?}", e);
                exit(1)
            }
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
                Executor::new(true_case).execute()
            }
            status if false_case.is_none() => Ok(status),
            _ => Executor::new(false_case.unwrap()).execute(),
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
                    Executor::new(command.clone()).execute()?;
                }
                _ => break,
            }
        }
        Ok(ExitStatus::new(0))
    }
}

fn split_env_and_commands(list: Vec<WordList>) -> (HashMap<String, String>, Vec<String>) {
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
                .to_string()
                .split_once("=")
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .unwrap()
        })
        .collect::<HashMap<_, _>>();
    let cmds = cmds
        .into_iter()
        .map(|wordlist| wordlist.to_string())
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

include!("command_test.rs");
