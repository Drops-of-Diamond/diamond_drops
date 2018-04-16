use fern;
use log::{LevelFilter};
use chrono;

use std::io;

fn setup_logger(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => {
            base_config
                .level(LevelFilter::Info)
                .level_for("overly-verbose-target", LevelFilter::Warn)
        }
        1 => base_config
            .level(LevelFilter::Debug)
            .level_for("overly-verbose-target", LevelFilter::Info),
        2 => base_config.level(LevelFilter::Debug),
        _3_or_more => base_config.level(LevelFilter::Trace),
    };

    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("output.log")?);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(io::stdout());

    base_config.chain(file_config).chain(stdout_config).apply()?;

    Ok(())
}

pub fn init() -> () {
    /*! Initialisation of [Log Crate](https://crates.io/crates/log) and [Fern Crate](https://docs.rs/fern/0.5.5/fern/) */
    /*! to output to both stdout and to a log file with choice of logging level macros from highest priority to lowest: */
    /*! `error!`, `warn!`, `info!`, `debug!` and `trace!`. */
    /*! [Compile time filters](https://docs.rs/log/0.4.1/log/#compile-time-filters) are configured in Cargo.toml */

    let verbosity: u64 = 2;

    match setup_logger(verbosity) {
        Ok(res) => { info!("Success initializing Rust Logger to verbosity level: {}", verbosity); () }
        Err(e) => { error!("Error initializing Rust Logger: {}", e); }
    }
}
