pub enum Mode {
    Proposer,
    Collator
}

impl Mode {
    pub fn value(&self) -> String {
        match *self {
            Mode::Proposer => "proposer".to_string(),
            Mode::Collator => "collator".to_string(),
        }
    }
}