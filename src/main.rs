use std::{env, fmt, fs, path::Path};

use libloading::{Library, Symbol};

use anigo::Plugin;

fn open(path: String) -> Result<(), libloading::Error> {
    println!("0");
    unsafe {
        let lib: Library = Library::new(path)?;

        println!("1");

        let plugin: Symbol<&'static Plugin> = lib.get(b"PLUGIN")?;

        println!("2");

        println!("{}", plugin.name);
    };

    Ok(())
}

fn main() {
    let p = Path::new(".").join("anigo/plugins/plugin.so");
    let pa = p.to_str();

    /*let path = Path::new(".")
    .join("anigo")
    .join("plugins")
    .join("plugin.so")
    .to_str();*/

    println!("{:#?}", open("./libanigo_consumet_plugin.so".to_string()));
}
