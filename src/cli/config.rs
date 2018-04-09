#[derive(Debug, PartialEq)]
pub enum Mode {
    Proposer,
    Collator,
    Both
}

impl Mode {
    pub fn value(&self) -> String {
        match *self {
            Mode::Proposer => "proposer".to_string(),
            Mode::Collator => "collator".to_string(),
            Mode::Both => "both".to_string()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub mode: Mode
}

impl Config {
    pub fn new(mode: Mode) -> Config {
        Config { mode }
    }
}
