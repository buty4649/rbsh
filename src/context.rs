#![allow(clippy::new_without_default)]

use super::{status::ExitStatus, syscall};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Clone)]
pub struct Context {
    inner: RefCell<ContextInner>,
}

#[derive(Debug, Clone)]
struct ContextInner {
    local_vars: HashMap<String, String>,
    status: ExitStatus,
    bin_name: String,
    positional_parameters: Vec<String>,
}

impl ContextInner {
    fn new() -> Self {
        Self {
            local_vars: HashMap::new(),
            status: ExitStatus::default(),
            bin_name: String::new(),
            positional_parameters: vec![],
        }
    }
}

impl Context {
    pub fn new() -> Self {
        let inner = ContextInner::new();

        Self {
            inner: RefCell::new(inner),
        }
    }

    //pub fn wrapper(&self) -> &Wrapper {
    //    &self.wrapper
    //}

    //pub fn env_vars(&self) -> HashMap<String, String> {
    //    self.wrapper.env_vars()
    //}

    pub fn set_status(&self, s: ExitStatus) {
        self.inner.borrow_mut().status = s
    }

    pub fn set_bin_name(&self, b: String) {
        self.inner.borrow_mut().bin_name = b
    }

    pub fn set_positional_parameters(&self, p: &[String]) {
        self.inner.borrow_mut().positional_parameters = p.to_vec();
    }

    pub fn set_var<T: AsRef<str>>(&self, name: T, value: T) -> Option<String> {
        let name = name.as_ref();
        let value = value.as_ref();

        let old_var = self.get_var(name);
        match syscall::env_get(name) {
            Ok(_) => syscall::env_set(name, value),
            Err(_) => {
                self.inner
                    .borrow_mut()
                    .local_vars
                    .insert(name.to_string(), value.to_string());
            }
        };

        old_var
    }

    pub fn get_status(&self) -> ExitStatus {
        self.inner.borrow().status
    }

    pub fn get_var<T: AsRef<str>>(&self, name: T) -> Option<String> {
        let name = name.as_ref();
        self.get_special_var(name)
            .or_else(|| self.get_env(name))
            .or_else(|| {
                self.inner
                    .borrow()
                    .local_vars
                    .get(name)
                    .map(|s| s.to_string())
            })
    }

    pub fn get_var_or_default<T: AsRef<str>>(&self, name: T, default: String) -> String {
        self.get_var(name).unwrap_or(default)
    }

    fn get_env<T: AsRef<str>>(&self, name: T) -> Option<String> {
        syscall::env_get(name.as_ref()).ok()
    }

    fn get_special_var<T: AsRef<str>>(&self, name: T) -> Option<String> {
        let mut c = name.as_ref().chars();
        match c.next() {
            Some('0') => Some(self.inner.borrow().bin_name.to_string()),
            Some(n) if n.is_ascii_digit() => {
                let mut index = n.to_digit(10).unwrap();
                for n in c {
                    if !n.is_ascii_digit() {
                        break;
                    }
                    index *= 10;
                    index += n.to_digit(10).unwrap();
                }
                self.inner
                    .borrow()
                    .positional_parameters
                    .get((index - 1) as usize)
                    .map(|s| s.to_string())
            }
            Some('?') => Some(self.inner.borrow().status.code().to_string()),
            Some('$') => {
                let pid = syscall::getpid();
                Some(format!("{}", pid))
            }
            _ => None,
        }
    }

    pub fn unset_var<T: AsRef<str>>(&self, name: T) -> Option<String> {
        let name = name.as_ref();
        let old_var = self.get_var(name);

        match syscall::env_get(name) {
            Ok(_) => syscall::env_unset(name),
            Err(_) => {
                self.inner.borrow_mut().local_vars.remove(name);
            }
        }

        old_var
    }
}
