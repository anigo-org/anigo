pub mod plugin;

use std::{env, error::Error, fs::create_dir_all};

use anigo::{self, specialty, Core, Manager, ManagerCycle};
use serde_json::Value;

fn init(core: &mut Core) -> Result<(), Box<dyn Error>> {
    let proyect_path = env::current_dir()?.join("anigo");
    let plugin_path = proyect_path.join("plugins");

    core.set(
        "proyect-path",
        Value::String(proyect_path.to_str().unwrap().to_string()),
    );

    core.set(
        "plugin-path",
        Value::String(plugin_path.to_str().unwrap().to_string()),
    );

    create_dir_all(plugin_path)?;

    let plugins = plugin::init::<fn ()>(core)?;

    for (lib, init) in plugins {
        init();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let core = &mut Core::new();

    println!("0");

    init(core)?;

    println!("1");

    let controllers = core.get_components(Some(&[&specialty::Component::Controller]));

    println!("2");

    let names = controllers
        .iter()
        .map(|x| x.name)
        .collect::<Vec<&'static str>>();

    println!("3");

    let resp = inquire::Select::new("", names).prompt()?;

    println!("You selected: {}", resp);

    Ok(())
}
