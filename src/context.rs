use super::{
    exec::syscall::{SysCallWrapper, Wrapper},
    status::ExitStatus,
};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Clone)]
pub struct Context {
    inner: RefCell<ContextInner>,
    wrapper: Wrapper,
}

#[derive(Debug, Clone)]
struct ContextInner {
    local_vars: HashMap<String, String>,
    status: ExitStatus,
}

impl ContextInner {
    fn new() -> Self {
        Self {
            local_vars: HashMap::new(),
            status: ExitStatus::default(),
        }
    }
}

impl Context {
    pub fn new(wrapper: Wrapper) -> Self {
        let inner = ContextInner::new();

        Self {
            inner: RefCell::new(inner),
            wrapper,
        }
    }

    pub fn wrapper(&self) -> &Wrapper {
        &self.wrapper
    }

    pub fn env_vars(&self) -> HashMap<String, String> {
        self.wrapper.env_vars()
    }

    pub fn set_staus(&self, s: ExitStatus) {
        self.inner.borrow_mut().status = s
    }

    pub fn set_var<T: AsRef<str>>(&self, name: T, value: T) -> Option<String> {
        let name = name.as_ref();
        let value = value.as_ref();

        let old_var = self.get_var(name);
        match self.wrapper.env_get(name) {
            Ok(_) => self.wrapper.env_set(name, value),
            Err(_) => {
                self.inner
                    .borrow_mut()
                    .local_vars
                    .insert(name.to_string(), value.to_string());
            }
        };

        old_var
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
        self.wrapper.env_get(name.as_ref()).ok()
    }

    fn get_special_var<T: AsRef<str>>(&self, name: T) -> Option<String> {
        match name.as_ref() {
            "?" => Some(self.inner.borrow().status.code().to_string()),
            "$" => {
                let pid = self.wrapper.getpid();
                Some(format!("{}", pid).to_string())
            }
            _ => None,
        }
    }

    pub fn unset_var<T: AsRef<str>>(&self, name: T) -> Option<String> {
        let name = name.as_ref();
        let old_var = self.get_var(name);

        match self.wrapper.env_get(name) {
            Ok(_) => self.wrapper.env_unset(name),
            Err(_) => {
                self.inner.borrow_mut().local_vars.remove(name);
            }
        }

        old_var
    }
}
