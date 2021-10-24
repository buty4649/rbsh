use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    local_vars: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            local_vars: HashMap::new(),
        }
    }

    pub fn env_vars(&self) -> HashMap<String, String> {
        env::vars().collect::<HashMap<_, _>>()
    }

    pub fn set_var(&mut self, name: &str, value: &str) {
        match env::var(&name) {
            Ok(_) => env::set_var(name, value),
            Err(_) => {
                self.local_vars.insert(name.to_string(), value.to_string());
            }
        };
    }

    pub fn get_var(&self, name: String) -> Option<String> {
        env::var(&name)
            .ok()
            .or_else(|| self.local_vars.get(&name).map(|s| s.to_string()))
    }
}
