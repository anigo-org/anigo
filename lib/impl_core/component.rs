use std::error::Error;

use crate::{specialty::Specialty, Component, Core, RComponent};

impl Core {
    pub fn extend_components(
        &mut self,
        library: &'static libloading::Library,
        data: Vec<Component>,
    ) -> Result<(), Box<dyn Error + '_>> {
        Ok(self.components.lock()?.push(RComponent {
            library,
            data,
        }))
    }

    pub fn get_components(
        &self,
        specs: Option<&'static [&'static dyn Specialty]>,
    ) -> Vec<Component> {
        let components = self.components.lock().unwrap().clone();

        let components = components.into_iter().map(|x| x.data).flatten();

        let components = components
            .filter(|x| {
                x.specialties
                    .iter()
                    .all(|spec| specs.unwrap_or(&[]).contains(spec))
            })
            .collect();

        components
    }
}
