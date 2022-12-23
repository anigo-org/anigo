use std::sync::{Mutex, Arc};

use crate::Core;

mod component;
mod library;
mod data;
mod life_cycle;

impl Core {
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(Vec::new())),
            libraries: Arc::new(Mutex::new(Vec::new())),
            data: Arc::new(Mutex::new(serde_json::Value::Null)),
            life_cycle: Arc::new(Mutex::new(Vec::new())),
            _libraries: vec![]
        }
    }
}
