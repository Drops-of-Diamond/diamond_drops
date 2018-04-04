use std::env;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Proposer,
    Collator,
    Both
}

#[derive(PartialEq)]
enum ConfigType {
    Mode,
    Nil
}

#[derive(Debug, PartialEq)]
pub struct Config {
    mode: Mode
}

impl Config {
    pub fn new(mode: Mode) -> Config {
        Config { mode }
    }
}

pub fn parse_args(mut args: Vec<String>) -> Result<Config, &'static str> {
    let mut config_type = ConfigType::Nil;
    
    // Default Case
    let mut mode = Mode::Both;

    for arg in args {
        // Match `-` prefixed args to a list of valid configuration points and set their values with the following non `-` prefixed value
        if arg.starts_with("-") {
            match arg.to_lowercase().as_ref() {
                    "-mode" => { config_type = ConfigType::Mode; },
                    _ => { return Err("Invalid configuration argument"); }
                }
        } else if config_type == ConfigType::Mode {
            // Match provided value to mode type
            match arg.to_lowercase().as_ref() {
                "proposer" | "p" => { mode = Mode::Proposer; },
                "collator" | "c" => { mode = Mode::Collator; },
                "both" | "b" => { mode = Mode::Both; }
                _ => { return Err("Invalid configuration value"); }
            }

            config_type = ConfigType::Nil;
        } else {
            return Err("No configuration argument supplied");
        }
    }

    if config_type == ConfigType::Nil {
        Ok(Config::new(mode))
    } else {
        Err("No configuration value supplied")
    }
}

pub fn run(config: Config) {
    /// The main function to run the node.  
    /// 
    /// # Inputs
    /// 
    /// config - A struct containing the configuration values for the client
    
    println!("Client Mode: {:?}", config.mode);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sets_client_mode_to_proposer() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("proposer")];
        let config_verbose = parse_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("p")];
        let config_short = parse_args(test_args_short).unwrap();

        assert_eq!(config_verbose.mode, Mode::Proposer);
        assert_eq!(config_short.mode, Mode::Proposer);
    }

    #[test]
    fn it_sets_client_mode_to_collator() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("collator")];
        let config_verbose = parse_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("c")];
        let config_short = parse_args(test_args_short).unwrap();

        assert_eq!(config_verbose.mode, Mode::Collator);
        assert_eq!(config_short.mode, Mode::Collator);
    }

    #[test]
    fn it_sets_client_mode_to_both() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("both")];
        let config_verbose = parse_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("b")];
        let config_short = parse_args(test_args_short).unwrap();
        // Default mode
        let test_args_default = vec![];
        let config_default = parse_args(test_args_default).unwrap();

        assert_eq!(config_verbose.mode, Mode::Both);
        assert_eq!(config_short.mode, Mode::Both);
        assert_eq!(config_default.mode, Mode::Both);
    }

    #[test]
    fn it_reports_invalid_arguments() {
        // Invalid configuration
        let test_args_configuration = vec![String::from("-bin"), String::from("collator")];
        let error_configuration = parse_args(test_args_configuration);

        // Invalid value
        let test_args_value = vec![String::from("-mode"), String::from("bin")];
        let error_value = parse_args(test_args_value);

        // No configuration
        let test_args_no_arg = vec![String::from("mode"), String::from("both")];
        let error_no_arg = parse_args(test_args_no_arg);

        // No value
        let test_args_no_value = vec![String::from("-mode")];
        let error_no_value = parse_args(test_args_no_value);

        assert_eq!(error_configuration, Err("Invalid configuration argument"));
        assert_eq!(error_value, Err("Invalid configuration value"));
        assert_eq!(error_no_arg, Err("No configuration argument supplied"));
        assert_eq!(error_no_value, Err("No configuration value supplied"));
    }
}
