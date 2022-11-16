use crate::CORE;

use std::env;

use anigo::Plugin;
use libloading::{Library, Symbol};
use walkdir::WalkDir;

pub fn open(path: &str) -> Result<(), libloading::Error> {
    let lib: Library = unsafe { Library::new(path)? };

    let plugin: Symbol<&'static Plugin> = unsafe { lib.get(b"PLUGIN")? };

    for entry in plugin.providers {
        unsafe { CORE.providers.push(*entry) };
    }

    for entry in plugin.workers {
        unsafe { CORE.workers.push(*entry) };
    }

    Ok(())
}

pub fn init(path: &str) -> Result<(), walkdir::Error> {
    for entry in WalkDir::new(path) {
        let entry = entry?;

        let name = entry.file_name().to_str().unwrap();
        let path = entry.path();

        if path.is_dir() || !name.starts_with(env::consts::DLL_EXTENSION) {
            continue;
        }

        let plugin = open(path.to_str().unwrap());

        if plugin.is_err() {
            continue;
        }
    }

    Ok(())
}
