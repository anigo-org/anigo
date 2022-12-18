pub mod plugin;

use std::error::Error;

use anigo::{runtime::Runtime, Func};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut life_cycle: Vec<Func> = Vec::new();

    life_cycle.push(plugin::init);
    life_cycle.push(plugin::main);

    let mut runtime: Runtime = Runtime::new(life_cycle);

    runtime.set("library-path", "plugins");

    for func in runtime.life_cycle.clone() {
        func(&mut runtime)?;
    }

    Ok(())
}
