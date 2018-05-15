extern crate diamond_drops;
extern crate diamond_drops_cli as cli;
extern crate diamond_drops_env as env;

#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_panic_running_client_mode_with_proposer() {
        env::config::set_test_env();
        let matches_short = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode").arg(Arg::with_name("proposer").short("p")))
            .get_matches_from(vec!["diamond-drops-cli", "mode", "-p"]);
        let test_args_short = matches_short;
        let config_short = cli::modules::args::process_mode_matches(&test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }

    #[test]
    fn it_does_not_panic_running_client_mode_with_notary() {
        env::config::set_test_env();
        let matches_short = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode").arg(Arg::with_name("notary").short("n")))
            .get_matches_from(vec!["diamond-drops-cli", "mode", "-n"]);
        let test_args_short = matches_short;
        let config_short = cli::modules::args::process_mode_matches(&test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }

    #[test]
    fn it_does_not_panic_running_client_mode_with_both() {
        env::config::set_test_env();
        env::config::set_test_env();
        let matches_short = App::new("diamond-drops-cli")
            .subcommand(SubCommand::with_name("mode").arg(Arg::with_name("both").short("b")))
            .get_matches_from(vec!["diamond-drops-cli", "mode", "-b"]);
        let test_args_short = matches_short;
        let config_short = cli::modules::args::process_mode_matches(&test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }
}
