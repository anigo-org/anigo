use std::sync::{MutexGuard, Arc, Mutex};

use crate::{specialty, Component, Core};

#[no_mangle]
pub fn mpv_1(core: Arc<Mutex<Core>>) {
    println!("mpv_1");
}

#[no_mangle]
pub fn init(mut core: MutexGuard<Core>) {
    let core = core.libraries.last_mut().unwrap();

    core.add(&[Component {
        name: "mpv",
        symbol: "mpv_1",
        description: "A flexible media player.",
        specialties: &[&specialty::Component::Controller],
        check: None,
    }]);
}

const EXPORTS: &'static [&'static Component] = &[
    &Component {
        name: "mpv",
        symbol: "mpv_1",
        description: "A flexible media player.",
        specialties: &[&specialty::Component::Controller],
        check: None,
    },
];
