use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use serde_json::Value;

//#[cfg(feature = "cdylib")]
//pub mod cdylib;

#[no_mangle]
pub fn init(m: &mut Manager) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub mod impl_core;
pub mod impl_manager;
pub mod specialty;

pub type ManagerCycle = fn(&mut Manager) -> Result<(), Box<dyn Error>>;
pub type LifeCycle = fn(&mut Core) -> Result<(), Box<dyn Error>>;

pub struct Core {
    _libraries: Vec<libloading::Library>,
    components: Arc<Mutex<Vec<RComponent>>>,
    data: Arc<Mutex<Value>>,
    libraries: Arc<Mutex<Vec<&'static libloading::Library>>>,
    life_cycle: Arc<Mutex<Vec<fn(&mut Core) -> Result<(), Box<dyn Error>>>>>,
}

pub struct Dependency {
    pub name: &'static str,
    pub description: &'static str,
    pub specialty: Box<dyn specialty::Specialty>,
    pub check: fn() -> Result<bool, Box<dyn Error>>,
}

#[derive(Clone)]
pub struct Component {
    pub name: &'static str,
    pub description: &'static str,
    pub specialties: &'static [&'static dyn specialty::Specialty],
    pub dependencies: &'static [specialty::Dependency],
}

#[derive(Clone)]
pub struct RComponent {
    library: &'static libloading::Library,
    data: Vec<Component>,
}

pub struct Manager<'c> {
    library: &'static libloading::Library,
    core: &'c mut Core,
}
