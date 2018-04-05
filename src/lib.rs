// Module declarations
pub mod cli;
pub mod config;
mod proposer;
mod collator;

use std::thread;

pub fn run(config: config::Config) {
    /// The main function to run the node.  
    /// 
    /// # Inputs
    /// 
    /// config - A struct containing the configuration values for the client
    
    println!("Client Mode: {:?}", config.mode);

    let proposer = proposer::Proposer::new();
    let collator = collator::Collator::new();

    match config.mode {
        config::Mode::Proposer => {
            println!("Running as a proposer");
            // Start a thread to run the proposer
            thread::spawn(move || {
                proposer.run();
            });
        },
        config::Mode::Collator => {
            println!("Running as a collator");
            // Start a thread to run the collator
            thread::spawn(move || {
                collator.run();
            });
        },
        config::Mode::Both => {
            println!("Running as both a proposer and collator");
            // Start threads for both proposer and collator
            thread::spawn(move || {
                proposer.run();
            });
            thread::spawn(move || {
                collator.run();
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sets_client_mode_to_proposer() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("proposer")];
        let config_verbose = cli::parse_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("p")];
        let config_short = cli::parse_args(test_args_short).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Proposer);
        assert_eq!(config_short.mode, config::Mode::Proposer);
    }

    #[test]
    fn it_sets_client_mode_to_collator() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("collator")];
        let config_verbose = cli::parse_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("c")];
        let config_short = cli::parse_args(test_args_short).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Collator);
        assert_eq!(config_short.mode, config::Mode::Collator);
    }

    #[test]
    fn it_sets_client_mode_to_both() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("both")];
        let config_verbose = cli::parse_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("b")];
        let config_short = cli::parse_args(test_args_short).unwrap();
        // Default mode
        let test_args_default = vec![];
        let config_default = cli::parse_args(test_args_default).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Both);
        assert_eq!(config_short.mode, config::Mode::Both);
        assert_eq!(config_default.mode, config::Mode::Both);
    }

    #[test]
    fn it_reports_invalid_arguments() {
        // Invalid configuration
        let test_args_configuration = vec![String::from("-bin"), String::from("collator")];
        let error_configuration = cli::parse_args(test_args_configuration);

        // Invalid value
        let test_args_value = vec![String::from("-mode"), String::from("bin")];
        let error_value = cli::parse_args(test_args_value);

        // No configuration
        let test_args_no_arg = vec![String::from("mode"), String::from("both")];
        let error_no_arg = cli::parse_args(test_args_no_arg);

        // No value
        let test_args_no_value = vec![String::from("-mode")];
        let error_no_value = cli::parse_args(test_args_no_value);

        assert_eq!(error_configuration, Err("Invalid configuration argument"));
        assert_eq!(error_value, Err("Invalid configuration value"));
        assert_eq!(error_no_arg, Err("No configuration argument supplied"));
        assert_eq!(error_no_value, Err("No configuration value supplied"));
    }
}
