//#![no_std]
#![feature(specialization)]
// #![feature(associated_type_defaults)]
//#![no_std]

// mod algebra;
mod catori;
mod nil;
mod path;
// mod universe;
// mod useful_types;
//
// pub use crate::algebra::*;
pub use crate::catori::*;
pub use crate::nil::*;
pub use crate::path::*;
// pub use crate::universe::*;
// pub use crate::useful_types::*;

mod logic_gates;
pub use logic_gates::*;
