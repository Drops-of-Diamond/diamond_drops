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
