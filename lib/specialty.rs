use std::fmt::Debug;
pub trait Specialty: Debug {}

impl PartialEq for dyn Specialty {
    fn eq(&self, other: &Self) -> bool {
        println!("{}", 2);
        self == other
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

impl Eq for dyn Specialty {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dependency {
    Development,
    Production,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Component {
    Controller,
    Memory,
    Solver,
}

impl Specialty for Dependency {}
impl Specialty for Component {}
