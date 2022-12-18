use std::{error::Error, env};

use anigo::runtime::Runtime;
use walkdir::WalkDir;

pub fn init(runtime: &mut Runtime) -> Result<(), Box<dyn Error>> {
    let path = runtime.get("library-path").unwrap().as_str().unwrap();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        let name = entry.file_name().to_str().unwrap();
        let path = entry.path();

        if path.is_dir() || !name.ends_with(env::consts::DLL_EXTENSION) {
            continue;
        }

        runtime.lp(path)?
    }

    Ok(())
}

pub fn main(runtime: &mut Runtime) -> Result<(), Box<dyn Error>> {
    
    Ok(())
}
