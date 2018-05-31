#![allow(unused_imports)]
#![allow(unused_assignments)]

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
    // TODO @dev see https://github.com/Drops-of-Diamond/diamond_drops/pull/90#pullrequestreview-124815845
    //error_chain! { }
}

pub mod modules;
