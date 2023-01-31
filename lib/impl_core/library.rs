use std::{ffi::OsStr, ops::Deref};

use libloading::Library;

use crate::{specialty::{self, Specialty}, Core, SharedLibrary};

impl Core {
    pub fn load_scontent<T, Y>(lib: &libloading::Library, sym: Y) -> Result<T, libloading::Error>
    where
        T: Clone,
        Y: AsRef<str>,
    {
        let data = unsafe { lib.get::<T>(sym.as_ref().as_bytes())? };
        let data = data.deref().clone();

        Ok(data)
    }

    pub fn load_slibrary<T>(path: T) -> Result<libloading::Library, libloading::Error>
    where
        T: AsRef<OsStr>,
    {
        unsafe { Library::new(path.as_ref()) }
    }
}

impl Core {
    pub fn load_content<T>(&self, specs: &'static [&'static dyn specialty::Specialty]) -> Vec<T>
    where
        T: Clone,
    {
        //let components = self.libraries.iter();

        let a: &dyn Specialty = &specialty::Component::Controller;
        let b: &[&dyn Specialty] = &[&specialty::Component::Controller];

        println!(
            "{:#?}", std::mem::size_of_val(&a)
        );

        println!(
            "{:#?}", std::mem::size_of_val(&b)
        );

        //for c in b.to_vec() {
            if b.last().unwrap() == &a {
                println!("1");
            }
        //}

        /*let libraries = components.map(|lib| {
            let shared = lib.shared.iter();

            let filtered = shared.filter(|value| {
                //let mut iter = value.specialties.iter();
                /*println!("3");
                //iter.all(|e| specs.contains(e))
                iter.last().unwrap() == specs.last().unwrap()*/

                /*let a: &dyn Specialty = &specialty::Component::Controller;
                let b: &[&dyn Specialty] = &[&specialty::Component::Controller];

                for spec in specs {
                    if !b.contains(
        &a
                    ) {
                        println!("{:#?}", value.specialties);
                        return false;
                    }
                }*/

                true
            });

            let loaded = filtered.map(|value| Box::new(Self::load_scontent::<T, _>(&lib.file, &value.name)));

            let loaded = loaded
                .filter(|value| value.is_ok())
                .map(|value| value.unwrap());

            println!("{:#?}", loaded);

            loaded
                .collect::<Vec<T>>()

        });

        libraries.flatten().collect::<Vec<_>>()*/
        Vec::new()
    }

    pub fn load_library<T>(&mut self, path: T) -> Result<&SharedLibrary, libloading::Error>
    where
        T: AsRef<OsStr>,
    {
        let library = Self::load_slibrary(path)?;
        let shared = SharedLibrary::new(library);

        self.libraries.push(shared);

        Ok(self.libraries.last().unwrap())
    }
}

impl Core {
    pub fn get_libraries(&self) -> &Vec<SharedLibrary> {
        &self.libraries
    }
}
