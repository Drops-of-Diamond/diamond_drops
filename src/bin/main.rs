extern crate diamond_drops;
use diamond_drops::Config;

use std::env;
use std::process;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let config = diamond_drops::parse_args(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    diamond_drops::run(config);
}