use crate::{specialty, Component, Manager};

impl Manager<'_> {
    pub fn extend_components(&mut self, data: &[Component]) {
        let _ = self.core.extend_components(self.library, data.to_vec());
    }

    pub fn get_components(
        &self,
        specs: Option<&'static [&'static dyn specialty::Specialty]>,
    ) -> Vec<Component> {
        self.core.get_components(specs)
    }
}
