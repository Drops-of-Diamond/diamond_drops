extern crate diamond_drops;

extern crate fern;

#[macro_use]
extern crate log;
extern crate chrono;

use diamond_drops::cli;

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
