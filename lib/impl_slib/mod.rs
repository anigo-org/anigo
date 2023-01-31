use crate::Component;

use super::SharedLibrary;

impl SharedLibrary {
    pub fn add(&mut self, data: &[Component]) {
        self.shared.extend_from_slice(data)
    }
}

impl SharedLibrary {
    pub fn new(file: libloading::Library) -> Self {
        Self {
            shared: Vec::new(),
            file,
        }
    }
}
