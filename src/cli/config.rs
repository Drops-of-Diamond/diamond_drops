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
            _ => "both".to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_set_thread_name_for_config_arguments() {
        let test_proposer_name = Mode::Proposer.value();
        let test_collator_name = Mode::Collator.value();

        assert_eq!(test_proposer_name, "proposer".to_string());
        assert_eq!(test_collator_name, "collator".to_string());
    }
}
