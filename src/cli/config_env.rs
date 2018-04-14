use std::env;

pub fn get_test_env() -> String {
    let key = "TEST";
    match env::var(key) {
        Ok(val) => {
            println!("Found environment variable key {}: {:?}", key, val);
            val.to_string()
        },
        Err(e) => {
            println!("Error interpreting environment variable key {}: {}", key, e);
            "".to_string()
        },
    }
}

pub fn set_test_env() {
    // Set the environment variable key `TEST` to with value of "1"
    // when `cargo test` has been run, otherwise set it to "0"
    let key = "TEST";
    let enabled = "1";
    let disabled = "0";
    if let Some(arg0) = env::args().nth(0) {
        if arg0 == "target/debug/diamond_drops" {
            env::set_var(key, disabled);
            assert_eq!(env::var(key), Ok(disabled.to_string()));
        } else {
            env::set_var(key, enabled);
            assert_eq!(env::var(key), Ok(enabled.to_string()));
        }
    }
}

pub fn is_running_with_cargo_test() -> bool {
    if get_test_env() == "1" {
        true
    } else {
        false
    }
}
