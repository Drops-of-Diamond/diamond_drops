extern crate diamond_drops;
extern crate diamond_drops_cli;
extern crate diamond_drops_env;

#[macro_use]
extern crate log;

use std::env;
use std::process;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    println!("ARGS {:?}", args);
    diamond_drops_cli::modules::log::init_logger("4");
    diamond_drops_env::config::set_test_env();
    let config = diamond_drops_cli::modules::args::parse_cli_args(args)
        .unwrap_or_else(|err| {
            error!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    diamond_drops::run(config);
}
