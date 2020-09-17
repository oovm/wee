mod support_cargo;
mod support_npm;

use anyhow::Result;
use serde_json::Value as Json;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use toml::value::Table;
use toml::Value as Toml;

pub struct Store {
    secrets: HashMap<String, String>,
    scripts: HashMap<String, String>,
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
        store.check_npm().ok();
        //store.scripts.retain(|k, _| k.to_string().contains(' '));
        return store;
    }
    pub fn insert_script(&mut self, k: S, v: S)
    where
        S: Into<String>,
    {
        let key = k.into();
        match self.scripts.entry(key) {
            Entry::Occupied(e) => {
                println!("GG");
                *e.get_mut() = v
            }
            Entry::Vacant(e) => {
                e.insert(v.into());
            }
        }
    }
    pub fn insert_secret(&mut self, k: S, v: S)
    where
        S: Into<String>,
    {
        let key = k.into();
        match self.secrets.entry(key) {
            Entry::Occupied(e) => {
                println!("GG");
                *e.get_mut() = v
            }
            Entry::Vacant(e) => {
                e.insert(v.into());
            }
        }
    }
}

impl Store {
    pub fn show_scripts(&self, times: usize) -> bool {}
}
