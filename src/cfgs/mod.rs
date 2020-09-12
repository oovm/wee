mod support_cargo;

use std::collections::HashMap;
use std::fs;
use anyhow::Result;

use serde_json::Value as Json;
use toml::Value as Toml;
use toml::value::Table;

pub struct Store {
    pub secrets: HashMap<Box<str>, String>,
    pub scripts: HashMap<Box<str>, String>,
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
    pub fn read_configs() -> Store {
        let mut store = Store::default();
        store.check_cargo().ok();
        store.check_npm();
        //store.scripts.retain(|k, _| k.to_string().contains(' '));
        return store;
    }

    fn check_npm(&mut self) {
        if let Ok(ref o) = fs::read_to_string("package.json") {
            if let Ok(value) = serde_json::from_str::<Json>(o) {
                if let Json::Object(dict) = value {
                    if let Some(s) = dict.get("envs") {
                        if let Json::Object(m) = s {
                            for (k, v) in m {
                                if let Json::String(str) = v {
                                    self.secrets.insert(Box::from(k.clone()), str.clone());
                                }
                            }
                        }
                    }
                    if let Some(s) = dict.get("scripts") {
                        if let Json::Object(m) = s {
                            for (k, v) in m {
                                if let Json::String(str) = v {
                                    self.scripts.insert(Box::from(k.clone()), str.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
