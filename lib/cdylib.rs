use std::error::Error;

use crate::{Manager, Component, specialty};

// fn (&mut Manager) -> Result<(), Box<dyn Error>>

#[no_mangle]
pub fn init(m: &mut Manager) -> Result<(), Box<dyn Error>> {
    /*m.extend_components(&[
        Component {
            name: "Exit",
            description: "Provide a controller to exit.",
            specialties: &[
                &specialty::Component::Controller,
            ],
            dependencies: &[],
        }
    ]);*/

    Ok(())
}
