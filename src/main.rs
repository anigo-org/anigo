pub mod plugin;

use std::{error::Error, env};

use anigo::{runtime::{Root, Runtime}, Func};

use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut life_cycle: Vec<Func> = Vec::new();

    life_cycle.push(plugin::init);
    life_cycle.push(plugin::main);

    let mut root: Root = Root::new(life_cycle);
    let mut runtime: Runtime = Runtime::new(&mut root, "core");

    let proyect_path = env::current_dir()?.join("anigo");
    let plugin_path = proyect_path.join("plugins");

    // Initialize global data.
    runtime.set_data("proyect-folder-path", 
        Value::String(
            proyect_path.to_str().unwrap().to_string()
        )
    );

    runtime.set_data("plugin-folder-path", 
        Value::String(
            plugin_path.to_str().unwrap().to_string()
        )
    );

    // Create anigo and plugins folder if they don't exist.
    std::fs::create_dir_all(plugin_path)?;

    for func in runtime.get_life_cycle() {
        func(&mut runtime)?;
    }

    Ok(())
}
