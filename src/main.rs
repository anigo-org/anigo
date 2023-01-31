pub mod plugin;

use std::{
    cell::RefCell,
    env,
    error::Error,
    fs::create_dir_all,
    future::Future,
    rc::Rc,
    sync::{Arc, Mutex},
};

use anigo::{self, specialty, Core};

fn init(core: Arc<Mutex<Core>>) -> Result<(), Box<dyn Error>> {
    let proyect_path = env::current_dir()?.join("anigo");
    let plugin_path = proyect_path.join("plugins");

    create_dir_all(&plugin_path)?;

    plugin::init(core, plugin_path, "init")?;

    Ok(())
}

use std::backtrace::Backtrace;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    //let bt = Backtrace::capture();

    let core = Arc::new(Mutex::new(Core::new()));

    println!("{}", 1);

    // Initialize plugins.
    init(Arc::clone(&core))?;

    let c = core.lock().unwrap();

    let data = c.load_content::<fn(Arc<Mutex<Core>>)>(&[&specialty::Component::Controller]);
    drop(c);


    // I have only testing if works.
    for entry in data {
        entry(Arc::clone(&core));
    }

    //println!("{:?}", bt);

    Ok(())
}
