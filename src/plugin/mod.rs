use std::{env, error::Error};

use anigo::Core;
use serde_json::Value;
use walkdir::WalkDir;

pub fn init<T>(core: &mut Core) -> Result<Vec<(&'static libloading::Library, T)>, Box<dyn Error>>
where
    T: Clone,
{
    let alt_path = Value::String("plugins".to_string());

    let path = core.get("plugin-path").unwrap_or(alt_path);
    let path = path.as_str().unwrap();

    let mut exports = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        let name = entry.file_name().to_str().unwrap();
        let path = entry.path();

        if path.is_dir() || !name.ends_with(env::consts::DLL_EXTENSION) {
            continue;
        }

        println!("0-3");

        let library = core.load_library(path).unwrap();

        
        println!("0-4");

        exports.push((
            library,
            core.load_library_content_from::<T, &str>("init", library)
                .unwrap(),
        ));

        println!("0-5");
    }

    Ok(exports)
}
