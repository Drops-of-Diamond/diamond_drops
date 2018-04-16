extern crate diamond_drops;

extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;

use diamond_drops::cli;
use diamond_drops::cli::{args};

use std::env;
use std::process;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    cli::config_log::init();
    cli::config_env::set_test_env();

    let config = cli::args::parse_cli_args(args).unwrap_or_else(|err| {
        error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    diamond_drops::run(config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_panic_running_client_mode_with_proposer() {
        cli::config_env::set_test_env();
        let test_args_short = vec![String::from("-mode"), String::from("p")];
        let config_short = cli::args::parse_cli_args(test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }

    #[test]
    fn it_does_not_panic_running_client_mode_with_notary() {
        cli::config_env::set_test_env();
        let test_args_short = vec![String::from("-mode"), String::from("n")];
        let config_short = cli::args::parse_cli_args(test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }

    #[test]
    fn it_does_not_panic_running_client_mode_with_both() {
        cli::config_env::set_test_env();
        let test_args_short = vec![String::from("-mode"), String::from("b")];
        let config_short = cli::args::parse_cli_args(test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }
}