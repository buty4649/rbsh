use std::cell::RefCell;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct Context {
    local_vars: RefCell<HashMap<String, String>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            local_vars: RefCell::new(HashMap::new()),
        }
    }

    pub fn env_vars(&self) -> HashMap<String, String> {
        env::vars().collect::<HashMap<_, _>>()
    }

    pub fn set_var(&self, name: &str, value: &str) {
        match env::var(&name) {
            Ok(_) => env::set_var(name, value),
            Err(_) => {
                self.local_vars
                    .borrow_mut()
                    .insert(name.to_string(), value.to_string());
            }
        };
    }

    pub fn get_var(&self, name: String) -> Option<String> {
        env::var(&name)
            .ok()
            .or_else(|| self.local_vars.borrow().get(&name).map(|s| s.to_string()))
    }
}
