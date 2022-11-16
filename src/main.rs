mod plugins;

use crate::plugins::init;

use std::path::Path;

use anigo::{Provider, Worker};

pub struct Core {
    pub providers: Vec<&'static dyn Provider>,
    pub workers: Vec<&'static dyn Worker>,
}

pub static mut CORE: Core = Core {
    providers: Vec::new(),
    workers: Vec::new(),
};

fn main() {
    let path = Path::new(".").join("anigo").join("plugins");
    let path = path.to_str().unwrap();

    println!("{:#?}", init(path));
}
