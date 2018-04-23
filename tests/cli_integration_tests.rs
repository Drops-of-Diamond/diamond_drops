extern crate diamond_drops;
extern crate diamond_drops_cli as cli;
extern crate diamond_drops_env as env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_panic_running_client_mode_with_proposer() {
        env::config::set_test_env();
        let test_args_short = vec![String::from("-mode"), String::from("p")];
        let config_short = cli::modules::args::parse_cli_args(test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }

    #[test]
    fn it_does_not_panic_running_client_mode_with_notary() {
        env::config::set_test_env();
        let test_args_short = vec![String::from("-mode"), String::from("n")];
        let config_short = cli::modules::args::parse_cli_args(test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }

    #[test]
    fn it_does_not_panic_running_client_mode_with_both() {
        env::config::set_test_env();
        let test_args_short = vec![String::from("-mode"), String::from("b")];
        let config_short = cli::modules::args::parse_cli_args(test_args_short).unwrap();
        let result = diamond_drops::run(config_short);

        assert_eq!(result, ());
    }
}