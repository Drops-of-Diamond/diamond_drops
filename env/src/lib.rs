// https://github.com/rust-lang-nursery/error-chain/issues/245
// https://github.com/Drops-of-Diamond/diamond_drops/issues/86
#![allow(renamed_and_removed_lints)]
#![allow(unused_doc_comment)]

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
