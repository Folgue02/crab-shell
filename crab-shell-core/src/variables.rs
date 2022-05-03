use std::collections::HashMap;
use std::env;

/// Container for both environmental variables and custom variables
pub struct Variables {
    pub env_vars: HashMap<String, String>,
    pub custom_vars: HashMap<String, String>,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            env_vars: env::vars().collect::<HashMap<String, String>>(),
            custom_vars: HashMap::new(),
        }
    }
    /// Creates or changes the value of a custom variable
    pub fn set_var(&mut self, key: String, value: String) {
        self.custom_vars.insert(key, value);
    }
    /// Removes/Unsets a custom variables
    pub fn unset_var(&mut self, key: &String) -> Result<(), ()> {
        if !self.custom_vars.contains_key(key) {
            return Err(());
        } else {
            self.custom_vars.remove(key);
            return Ok(());
        }
    }
    pub fn refresh_env_vars(&mut self) {
        self.env_vars = env::vars().collect::<HashMap<String, String>>();
    }
}