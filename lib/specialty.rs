use std::fmt::Debug;

pub trait Specialty: Debug {}

impl PartialEq for dyn Specialty {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dependency {
    Development,
    Production,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Component {
    Controller,
    Solver,
}

impl Specialty for Dependency {}
impl Specialty for Component {}
