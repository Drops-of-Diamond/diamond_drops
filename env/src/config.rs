extern crate error_chain;

use errors;
use std::env;

pub fn get_env() -> String {
    let key = "RUST_ENV";
    match env::var(key) {
        Ok(val) => {
            debug!("Found environment variable key {}: {:?}", key, val);
            val.to_string()
        },
        Err(e) => {
            error!("Error interpreting environment variable key {}: {}", key, e);
            "".to_string()
        },
    }
}

pub fn set_test_env() {
    // Set the environment variable key `TEST` to value of "1"
    // when `cargo test` has been run, otherwise set it to "0"
    let key = "RUST_ENV";
    let value_test = "TEST";
    let value_development = "DEVELOPMENT";
    if let Some(arg0) = env::args().nth(0) {
        if arg0 == "target/debug/diamond_drops" {
            env::set_var(key, value_development);
            assert_eq!(env::var(key), Ok(value_development.to_string()));
        } else {
            env::set_var(key, value_test);
            assert_eq!(env::var(key), Ok(value_test.to_string()));
        }
    }
}

pub fn is_running_with_cargo_test() -> bool {
    if get_env() == "TEST" {
        true
    } else {
        false
    }
}
