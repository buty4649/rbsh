use super::exec::syscall::{SysCallWrapper, Wrapper};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    local_vars: HashMap<String, String>,
    wrapper: Wrapper,
}

impl Context {
    pub fn new(wrapper: Wrapper) -> Self {
        Self {
            local_vars: HashMap::new(),
            wrapper,
        }
    }

    pub fn wrapper(&self) -> &Wrapper {
        &self.wrapper
    }

    pub fn env_vars(&self) -> HashMap<String, String> {
        self.wrapper.env_vars()
    }

    pub fn set_var(&mut self, name: &str, value: &str) {
        match self.wrapper.env_get(name) {
            Ok(_) => self.wrapper.env_set(name, value),
            Err(_) => {
                self.local_vars.insert(name.to_string(), value.to_string());
            }
        };
    }

    pub fn get_var(&self, name: String) -> Option<String> {
        self.wrapper
            .env_get(&name)
            .ok()
            .or_else(|| self.local_vars.get(&name).map(|s| s.to_string()))
    }
}
