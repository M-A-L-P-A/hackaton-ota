// Can work in an embedded env, as long as there exists the concept of a heap:
#![no_std]
extern crate alloc;

pub mod structs;
pub mod traits;
pub mod errors;
pub mod core;
