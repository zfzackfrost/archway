#![no_std]

extern crate alloc;

mod kind;
mod shared;
mod weak;

pub use kind::*;
pub use shared::*;
pub use weak::*;
