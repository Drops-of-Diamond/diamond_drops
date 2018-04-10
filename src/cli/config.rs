#[derive(Debug, PartialEq)]
pub enum Mode {
    Proposer,
    Notary,
    Both
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