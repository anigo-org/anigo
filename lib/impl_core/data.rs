use serde_json::Value;

use crate::Core;

impl Core {
    pub fn get(&self, key: &str) -> Option<Value> {
        self.data.lock().unwrap().get(key).cloned()
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.data.lock().unwrap()[key] = value;
    }
}
