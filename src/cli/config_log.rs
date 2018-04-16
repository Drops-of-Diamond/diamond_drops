use fern;
use log;
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use chrono::Local;

use std::io;

static LOGGER: DiamondDropsLogger = DiamondDropsLogger;

struct DiamondDropsLogger;

impl log::Log for DiamondDropsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} [{}] - {}", Local::now().format("[%Y-%m-%d][T%H:%M:%S]"), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][T%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

pub fn init() -> () {
    /*! Initialisation of [Log Crate](https://crates.io/crates/log) and [Fern Crate](https://docs.rs/fern/0.5.5/fern/) */
    /*! with choice of logging level macros from highest priority to lowest: `error!`, `warn!`, `info!`, `debug!` and `trace!`. */
    /*! [Compile time filters](https://docs.rs/log/0.4.1/log/#compile-time-filters) are configured in Cargo.toml */

    setup_logger();
}
