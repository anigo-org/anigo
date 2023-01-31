use std::error::Error;

pub mod impl_core;
pub mod impl_slib;
pub mod specialty;

pub mod cdylib;

pub const PLUGIN_PATH: &str = "plugins";

#[derive(Clone, Debug, PartialEq)]
pub struct Component {
    pub name: &'static str,
    pub symbol: &'static str,
    pub description: &'static str,
    pub specialties: &'static [&'static dyn specialty::Specialty],
    pub check: Option<fn() -> Result<bool, Box<dyn Error>>>,
}

#[derive(Debug)]
pub struct SharedLibrary {
    pub shared: Vec<Component>,
    pub file: libloading::Library,
}

#[derive(Debug)]
pub struct Core {
    libraries: Vec<SharedLibrary>,
}
