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
    list: CommandList,
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
    pub fn new(list: CommandList) -> Self {
        Self { list }
    }

    pub fn execute(&mut self) -> Result<ExitStatus> {
        let status = match self.list.next() {
            Some(c) => self.execute_command(c),
            None => Ok(ExitStatus::new(0)), // noop
        };

        match status {
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
            Ok(s) => Ok(s),
        }
    }

    fn execute_command(&self, cmd: UnitKind) -> Result<ExitStatus> {
        match cmd {
            UnitKind::SimpleCommand {
                command,
                redirect,
                background,
            } => self.execute_simple_command(command, redirect, background),
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
