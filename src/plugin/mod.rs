use std::{error::Error, env, process};

use anigo::{runtime::{Runtime, Specialty}, plugin, Func};
use walkdir::WalkDir;

pub fn init(runtime: &mut Runtime) -> Result<(), Box<dyn Error>> {
    let path = runtime.get_data("plugin-folder-path").as_str().unwrap();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        let name = entry.file_name().to_str().unwrap();
        let path = entry.path();

        if path.is_dir() || !name.ends_with(env::consts::DLL_EXTENSION) {
            continue;
        }

        runtime.load_plugin(path)?;
    }

    Ok(())
}

use inquire;

pub fn main(r: &mut Runtime) -> Result<(), Box<dyn Error>> {
    let target: Box<dyn Specialty> = Box::new(plugin::Specialty::Controller); 
    let mut controllers: Vec<(&str, Func)> = Vec::new();
    controllers.push(("Exit", |_| {
        process::exit(1)
    }));

    for plugin in r.get_plugins() {
        for addon in plugin.1.iter() {
            if *addon.specialty != *target { continue }

            let controller: Func = r.load_plugin_content(plugin.0, addon.symbol)?;

            controllers.push((addon.name, controller));
        }
    }

    let c = controllers.into_iter().map(|(name, _)| name).collect::<Vec<&str>>();

    inquire::Select::new("Select a controller", c).prompt()?;

    Ok(())
}
