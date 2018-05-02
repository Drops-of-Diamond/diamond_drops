//! ![uml](ml.svg)
#![feature(generator_trait)]
#![allow(unused_imports)] // Only use this while testing, otherwise comment out,
// especially before releasing.

extern crate diamond_drops_cli as cli;

extern crate ethereum_types;
extern crate tiny_keccak;
#[macro_use]
extern crate log;

pub mod modules;