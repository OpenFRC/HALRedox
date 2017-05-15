#[macro_use]
extern crate error_chain;
extern crate futures;

pub mod hal;

#[cfg(target_arch="arm")]
pub use hal::*;
