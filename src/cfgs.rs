use std::collections::HashMap;
use std::fs;
use std::io::Error;
use toml::Value;

pub struct Store {
    secrets: HashMap<Box<str>, String>,
    scripts: HashMap<Box<str>, String>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            secrets: Default::default(),
            scripts: Default::default(),
        }
    }
}

impl Store {
    #[test]
    pub fn read_configs() {
        let mut store = Store::default();
        store.try_rust();
        store.try_js();
    }
    fn try_rust(mut self) {
        match fs::read_to_string("Cargo.toml") {
            Ok(o) => {
                match o.parse::<Value>() {
                    Ok(value) => {
                        match value {
                            Value::Table(dict) => {
                                match dict.get("env") {
                                    None => {}
                                    Some(_) => {}
                                }
                                match dict.get("scripts") {
                                    None => {}
                                    Some(s) => {
                                        match s {
                                            Value::Table(m) => {
                                                for (k, v) in m {
                                                    match v {
                                                        Value::String(str) => {
                                                            self.scripts.insert(Box::from(k), str.clone())
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        };
    }
    fn try_js(mut self) {
        match fs::read_to_string("package.json") {
            Ok(o) => {}
            _ => {}
        }
    }
}
