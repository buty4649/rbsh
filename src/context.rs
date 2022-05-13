#![allow(clippy::new_without_default)]

use super::{status::ExitStatus, syscall};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    local_vars: HashMap<String, String>,
    pub status: ExitStatus,
    pub bin_name: String,
    pub positional_parameters: Vec<String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            local_vars: HashMap::new(),
            status: ExitStatus::default(),
            bin_name: String::new(),
            positional_parameters: vec![],
        }
    }

    pub fn set_var<T: AsRef<str>>(&mut self, name: T, value: T) -> Option<String> {
        let name = name.as_ref();
        let value = value.as_ref();

        let old_var = self.get_var(name);
        match syscall::env_get(name) {
            Ok(_) => syscall::env_set(name, value),
            Err(_) => {
                self.local_vars.insert(name.to_string(), value.to_string());
            }
        };

        old_var
    }

    pub fn get_var<T: AsRef<str>>(&self, name: T) -> Option<String> {
        let name = name.as_ref();
        self.get_special_var(name)
            .or_else(|| syscall::env_get(name).ok())
            .or_else(|| self.local_vars.get(name).map(|s| s.to_string()))
    }

    pub fn get_var_or_default<T: AsRef<str>>(&self, name: T, default: String) -> String {
        self.get_var(name).unwrap_or(default)
    }

    fn get_special_var<T: AsRef<str>>(&self, name: T) -> Option<String> {
        let mut c = name.as_ref().chars();
        match c.next() {
            Some('0') => Some(self.bin_name.to_string()),
            Some(n) if n.is_ascii_digit() => {
                let mut index = n.to_digit(10).unwrap();
                for n in c {
                    if !n.is_ascii_digit() {
                        break;
                    }
                    index *= 10;
                    index += n.to_digit(10).unwrap();
                }
                self.positional_parameters
                    .get((index - 1) as usize)
                    .map(|s| s.to_string())
            }
            Some('?') => Some(self.status.code().to_string()),
            Some('$') => {
                let pid = syscall::getpid();
                Some(format!("{}", pid))
            }
            _ => None,
        }
    }

    pub fn unset_var<T: AsRef<str>>(&mut self, name: T) -> Option<String> {
        let name = name.as_ref();
        let old_var = self.get_var(name);

        match syscall::env_get(name) {
            Ok(_) => syscall::env_unset(name),
            Err(_) => {
                self.local_vars.remove(name);
            }
        }

        old_var
    }
}
