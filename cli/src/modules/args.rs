use modules::{config, log};

use clap;
use clap::{App, Arg, SubCommand};

#[derive(PartialEq)]
enum ConfigType {
    Mode,
    Nil
}

/// Separate function to test functionality of matching CLI args of mode independent of logger
pub fn process_mode_matches(matches: &clap::ArgMatches) -> Result<config::Config, String> {
    let mut config_type = ConfigType::Nil;

    // Default Case
    let mut mode = config::Mode::Both;

    // Handle subcommand info by conditionally getting the value and requesting matches by name
    match matches.subcommand() {
        ("mode", Some(subcommand_matches)) => {
            info!("Detected mode config...");
            if subcommand_matches.is_present("proposer") || matches.is_present("p") {
                info!("Proposer mode activated...");
                mode = config::Mode::Proposer;
            } else if subcommand_matches.is_present("notary") || matches.is_present("n") {
                info!("Notary mode activated...");
                mode = config::Mode::Notary;
            } else if subcommand_matches.is_present("both") || matches.is_present("b") {
                info!("Both Proposer and Notary modes activated...");
                mode = config::Mode::Both;
            } else {
                let error_msg = msg_with_args("Invalid mode argument provided");
                return Err(error_msg);
            }
            config_type = ConfigType::Mode;
        },
        _ => {
            let error_msg = msg_with_args("No mode argument provided");
            return Err(error_msg);
        }
    }

    if config_type == ConfigType::Mode {
        Ok(config::Config::new(mode))
    } else {
        let error_msg = msg_with_args("Invalid mode argument provided");
        return Err(error_msg);
    }
}

/// Separate function to test functionality of matching CLI args of logger independent of mode
fn process_log_matches(matches: &clap::ArgMatches) -> () {
    let default_trace_log_level: &str = &String::from("4");
    let log_pattern = matches.value_of("log").unwrap_or(default_trace_log_level);
    info!("Config log pattern: {}", log_pattern);
    log::init_logger(&log_pattern);
    ()
}

/// Parse arguments from the command line and produce a configuration from them.
pub fn parse_cli_args(_args: Vec<String>) -> Result<config::Config, String> {
    let yaml = load_yaml!("../../cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    process_log_matches(&matches);

    let mode = process_mode_matches(&matches).unwrap();

    return Ok(mode);
}

fn msg_with_args(msg: &str) -> String {
    let avail_args: &str = ", try cargo run -- mode --help";
    let mut ret = msg.to_string().to_owned();
    ret.push_str(avail_args);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sets_client_mode_to_proposer() {
        let matches_long = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode")
                .arg(Arg::with_name("proposer")
                    .long("proposer")
                ))
            .get_matches_from(vec![
                "diamond-drops-cli", "mode", "--proposer"
            ]);

        let matches_short = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode")
                .arg(Arg::with_name("proposer")
                    .short("p")
                ))
            .get_matches_from(vec![
                "diamond-drops-cli", "mode", "-p"
            ]);

        // Verbose command
        let test_args_verbose = matches_long;
        let config_verbose = process_mode_matches(&test_args_verbose).unwrap();

        // Short command
        let test_args_short = matches_short;
        let config_short = process_mode_matches(&test_args_short).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Proposer);
        assert_eq!(config_short.mode, config::Mode::Proposer);
    }

    #[test]
    fn it_sets_client_mode_to_notary() {
        let matches_long = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode")
                .arg(Arg::with_name("notary")
                    .long("notary")
                ))
            .get_matches_from(vec![
                "diamond-drops-cli", "mode", "--notary"
            ]);

        let matches_short = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode")
                .arg(Arg::with_name("notary")
                    .short("n")
                ))
            .get_matches_from(vec![
                "diamond-drops-cli", "mode", "-n"
            ]);

        // Verbose command
        let test_args_verbose = matches_long;
        let config_verbose = process_mode_matches(&test_args_verbose).unwrap();

        // Short command
        let test_args_short = matches_short;
        let config_short = process_mode_matches(&test_args_short).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Notary);
        assert_eq!(config_short.mode, config::Mode::Notary);
    }

    #[test]
    fn it_sets_client_mode_to_both() {
        let matches_long = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode")
                .arg(Arg::with_name("both")
                    .long("both")
                ))
            .get_matches_from(vec![
                "diamond-drops-cli", "mode", "--both"
            ]);

        let matches_short = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode")
                .arg(Arg::with_name("both")
                    .short("b")
                ))
            .get_matches_from(vec![
                "diamond-drops-cli", "mode", "-b"
            ]);

        // Verbose command
        let test_args_verbose = matches_long;
        let config_verbose = process_mode_matches(&test_args_verbose).unwrap();

        // Short command
        let test_args_short = matches_short;
        let config_short = process_mode_matches(&test_args_short).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Both);
        assert_eq!(config_short.mode, config::Mode::Both);
    }

    #[test]
    fn it_sets_logger_with_verbose_argument() {
        let matches_long = App::new("diamond-drops-cli")
            .arg(Arg::with_name("log")
                .long("log")
                .required(false)
                .takes_value(true))
            .get_matches_from(vec![
                "diamond-drops-cli", "--log", "4"
            ]);

        // Verbose command
        let test_args_verbose = matches_long;
        let config_verbose = process_log_matches(&test_args_verbose);

        assert_eq!(config_verbose, ());
    }

    #[test]
    fn it_sets_logger_without_short_argument() {
        let matches_short = App::new("diamond-drops-cli")
            .arg(Arg::with_name("log")
                .short("l")
                .required(false)
                .takes_value(true))
            .get_matches_from(vec![
                "diamond-drops-cli", "-l", "4"
            ]);

        // Short command
        let test_args_short = matches_short;
        let config_short = process_log_matches(&test_args_short);

        assert_eq!(config_short, ());
    }

    #[test]
    fn it_reports_invalid_arguments() {
        let mut error_msg = "".to_string();

        // Missing subcommand
        let matches_invalid_subcommand = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode")
                .arg(Arg::with_name("notary")
                    .long("notary")
                ))
            .get_matches_from(vec![
                "diamond-drops-cli"
            ]);
        let test_args_configuration = matches_invalid_subcommand;
        let error_configuration = process_mode_matches(&test_args_configuration);
        error_msg = msg_with_args("No mode argument provided");

        assert_eq!(error_configuration, Err(error_msg));

        // No mode argument
        let matches_invalid_subcommand = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode")
                .arg(Arg::with_name("notary")
                    .long("notary")
                ))
            .get_matches_from(vec![
                "diamond-drops-cli", "mode"
            ]);
        let test_args_no_arg = matches_invalid_subcommand;
        let error_no_arg = process_mode_matches(&test_args_no_arg);
        error_msg = msg_with_args("Invalid mode argument provided");

        assert_eq!(error_no_arg, Err(error_msg));
    }

    #[test]
    fn it_appends_available_arguments_postfix_to_error_message_prefix() {
        // Expected
        let expected_output: &str = "Invalid something or other, try cargo run -- mode --help";
        let expected_error_msg_output = String::from(expected_output);

        // Actual
        let actual_error_msg_prefix_input = String::from("Invalid something or other");
        let actual_error_msg_output = msg_with_args(&actual_error_msg_prefix_input);

        assert_eq!(expected_error_msg_output, actual_error_msg_output);
    }
}