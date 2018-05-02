// UML Generator https://github.com/adjivas/ml
extern crate mml;

fn main() {
    // Generate diagram in /diagrams/ml.svg for README.md
    let dest: String = "../diagrams/".to_string();
    let _ = mml::src2both("src", dest.replace("-", "_").as_str());

    // Generate diagram in /target/doc/diamond_drops_node/ml.svg
    let dest2: String = concat!("../target/doc/", env!("CARGO_PKG_NAME")).to_string();
    let _ = mml::src2both("src", dest.replace("-", "_").as_str());
}