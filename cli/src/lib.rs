#![allow(unused_imports)]
#![allow(unused_assignments)]
// https://github.com/rust-lang-nursery/error-chain/issues/245
// https://github.com/Drops-of-Diamond/diamond_drops/issues/86
#![allow(renamed_and_removed_lints)]
#![allow(unused_doc_comment)]

extern crate chrono;
extern crate env_logger;
extern crate fern;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

pub mod error {
    error_chain! { }
}

pub mod modules;
