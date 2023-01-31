mod library;

use super::Core;

impl Core {
    pub fn new() -> Self {
        Self {
            libraries: Vec::new(),
        }
    }
}
