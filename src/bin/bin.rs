// Simple and robust error handling with error-chain!

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Remove these allow warnings before a production release
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate diamond_drops;
extern crate diamond_drops_cli;
extern crate diamond_drops_env;

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use std::env;
use std::process;

pub use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        process::exit(1);
    }
}

// Use this macro to auto-generate the main above. You may want to
// set the `RUST_BACKTRACE` env variable to see a backtrace.
// quick_main!(run);

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.

fn run() -> Result<()> {

    let args = env::args().skip(1).collect::<Vec<_>>();
    println!("Processing arguments: {:?}", args);
    diamond_drops_env::config::set_test_env();
    let config = diamond_drops_cli::modules::args::parse_cli_args(args)
        .unwrap_or_else(|err| {
            error!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    diamond_drops::run(config);

    Ok(())
}
