//! ![uml](ml.svg)

//unused: #![feature(slice_get_slice)]

// Make sure to actually handle these before production. 
// https://github.com/Drops-of-Diamond/diamond_drops/issues/66
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// https://github.com/rust-lang-nursery/error-chain/issues/245
// https://github.com/Drops-of-Diamond/diamond_drops/issues/86
#![allow(renamed_and_removed_lints)]
#![allow(unused_doc_comment)]

//extern crate bitreader;
extern crate diamond_drops_cli as cli;
extern crate ethereum_types;
extern crate tiny_keccak;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

pub mod modules;