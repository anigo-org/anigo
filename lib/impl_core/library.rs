use std::{error::Error, ffi::OsStr, mem::transmute, ops::Deref};

use libloading::Library;

use crate::{specialty, Core, ManagerCycle, Manager};

impl Core {
    pub fn load_library_content_from<T, Y>(
        &self,
        sym: Y,
        lib: &'static libloading::Library,
    ) -> Result<T, Box<dyn Error + '_>>
    where
        T: Clone,
        Y: AsRef<str>,
    {
        let sym = sym.as_ref();

        println!("{:#?}", lib.to_owned());

        let res = unsafe { lib.get::<T>(b"init") };

        println!("1");
        println!("{:#?}", res);

        let val = res?.deref().clone();

        Ok(val)
    }

    pub fn load_library_content<T>(
        &self,
        specs: &'static [&'static dyn specialty::Specialty],
    ) -> Result<Vec<T>, Box<dyn Error + '_>>
    where
        T: Clone,
    {
        let components = self.components.lock()?;

        let components = components.iter().map(|element| {
            let data = element
                .data
                .iter()
                .filter(|comp| comp.specialties.iter().all(|spec| specs.contains(spec)));

            (element.library, data)
        });

        let components = components.map(|(lib, data)| {
            data.map(|i1| Ok(self.load_library_content_from(i1.name, lib)?))
        });

        let components = components
            .flatten()
            .collect::<Result<Vec<T>, Box<dyn Error + '_>>>()?;

        Ok(components)
    }

    pub fn load_library<T>(
        &mut self,
        path: T,
    ) -> Result<&'static libloading::Library, Box<dyn Error + '_>>
    where
        T: AsRef<OsStr>,
    {
        self._libraries.push(unsafe {
            libloading::Library::new(path.as_ref())?
        });

        Ok(self._libraries.last().unwrap())
    }

}

impl Core {
    pub fn get_libraries(&self) -> Vec<&'static libloading::Library> {
        self.libraries.lock().unwrap().clone()
    }
}
