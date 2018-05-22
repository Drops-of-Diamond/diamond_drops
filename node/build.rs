// UML Generator https://github.com/adjivas/ml

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate mml;

/* #[macro_use]
extern crate error_chain;

pub mod errors { error_chain!{} }

use errors::*;
 */

fn make_uml_diagram() {
    // Generate diagram in /diagrams/ml.svg for README.md
    let dest: String = "../diagrams/".to_string();
    let _ = mml::src2both("src", dest.replace("-", "_").as_str());

    // Generate diagram in /target/doc/diamond_drops_node/ml.svg
    let dest: String = concat!("../target/doc/", env!("CARGO_PKG_NAME")).to_string();
    let _ = mml::src2both("src", dest.replace("-", "_").as_str());
}

fn main() {
    //make_uml_diagram();
}
