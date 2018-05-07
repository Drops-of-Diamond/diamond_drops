pub mod args;
pub mod config;
pub mod log;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}