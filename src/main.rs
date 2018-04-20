/* Copyright 2018 James Ray (@jamesray1), Josiah Evans (@ChosunOne), Luke Schoen (@ltfschoen)

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE 
AUTHORS, James Ray, Josiah @ChosunOne, and Luke Schoen
BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
 
For more information, please refer to <http://unlicense.org>
*/

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
