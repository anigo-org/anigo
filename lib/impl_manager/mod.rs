pub mod component;
pub mod library;

use crate::{Core, Manager};

impl<'c> Manager<'c> {
    pub fn new(library: &'static libloading::Library, core: &'c mut Core) -> Self {
        Self { library, core }
    }
}
