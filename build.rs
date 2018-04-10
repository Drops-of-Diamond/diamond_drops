// UML Generator https://github.com/adjivas/ml
extern crate mml;

fn main() {
    let dest: String = "diagrams/".to_string();

    let _ = mml::src2both("src", dest.replace("-", "_").as_str());
}