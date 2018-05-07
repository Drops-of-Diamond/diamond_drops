#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

pub mod config;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

pub use errors::*;