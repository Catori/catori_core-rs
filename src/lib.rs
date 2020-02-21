#![no_std]
#![feature(specialization)]
#![feature(associated_type_defaults)]
//#![no_std]

mod path;
mod catori;
mod universe;
mod nil;
mod algebra;
mod useful_types;

pub use crate::path::*;
pub use crate::catori::*;
pub use crate::universe::*;
pub use crate::nil::*;
pub use crate::algebra::*;
pub use crate::useful_types::*;

