use std::{
    env,
    error::Error,
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

use anigo::Core;
use walkdir::WalkDir;

pub fn init<T>(core: Arc<Mutex<Core>>, path: PathBuf, symbol: T) -> Result<(), Box<dyn Error>>
where
    T: AsRef<str>,
{
    for entry in WalkDir::new(path) {
        let entry = entry?;

        let filename = entry.file_name().to_str().unwrap();
        let path = entry.path();

        if path.is_dir() || !filename.ends_with(env::consts::DLL_EXTENSION) {
            continue;
        }

        let core = Arc::clone(&core);
        let mut core = core.lock().unwrap();

        match core.load_library(path) {
            Ok(shared) => {
                match Core::load_scontent::<fn(MutexGuard<Core>), _>(&shared.file, &symbol) {
                    Err(e) => println!("Failed to load library: {}\n{}", filename, e),
                    Ok(e) => e(core),
                }
            }
            Err(e) => println!("Failed to load library: {}\n{}", filename, e),
        }
    }

    Ok(())
}
