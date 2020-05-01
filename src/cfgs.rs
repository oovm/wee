use std::collections::HashMap;
use std::fs;
use toml::Value;

pub struct Store {
    secrets: HashMap<Box<str>, String>,
    env: HashMap<Box<str>, String>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            secrets: Default::default(),
            env: Default::default(),
        }
    }
}

impl Store {
    #[test]
    pub fn read_configs() {
        let mut store = Store::default();
        store.try_rust();
    }
    fn try_rust(mut self) {
        let contents = fs::read_to_string("Cargo.toml")?;
        let value = contents.parse::<Value>().unwrap();
        match value {
            Value::Table(_) => {}
            _ => {}
        }
    }
}
