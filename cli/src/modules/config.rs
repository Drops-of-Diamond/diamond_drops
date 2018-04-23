/// The mode the client is running in
#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Proposer,
    Notary,
    Both
}

impl Mode {
    /// Returns a string value of the current running mode
    pub fn value(&self) -> String {
        match *self {
            Mode::Proposer => "proposer".to_string(),
            Mode::Notary => "notary".to_string(),
            _ => "both".to_string()
        }
    }
}

/// This holds configuration options for running the client
#[derive(Debug, PartialEq)]
pub struct Config {
    pub mode: Mode
}

impl Config {
    /// Creates a new configuration to be run
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
        let test_notary_name = Mode::Notary.value();

        assert_eq!(test_proposer_name, "proposer".to_string());
        assert_eq!(test_notary_name, "notary".to_string());
    }
}
