use std::error::Error;

use crate::{specialty, Manager};

impl Manager<'_> {
    pub fn load_library_content<T>(
        &self,
        specs: &'static [&'static dyn specialty::Specialty],
    ) -> Result<Vec<T>, Box<dyn Error + '_>>
    where
        T: Clone,
    {
        Ok(self.core.load_library_content(specs)?)
    }
}

impl Manager<'_> {
    pub fn get_libraries(&self) -> Result<Vec<&'static libloading::Library>, Box<dyn Error + '_>> {
        Ok(self.core.libraries.lock()?.clone())
    }
}
