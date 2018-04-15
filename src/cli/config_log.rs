use log;
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

static LOGGER: DiamondDropsLogger = DiamondDropsLogger;

struct DiamondDropsLogger;

impl log::Log for DiamondDropsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> () {
    /*! Initialisation of [Log Crate](https://crates.io/crates/log) with choice of logging level macros */
    /*! from highest priority to lowest: `error!`, `warn!`, `info!`, `debug!` and `trace!`. */
    /*! [Compile time filters](https://docs.rs/log/0.4.1/log/#compile-time-filters) are configured in Cargo.toml */

    let logger = log::set_logger(&LOGGER);
    match logger {
        Ok(res) => {
            log::set_max_level(LevelFilter::Trace);
            eprintln!("Success initializing Rust Logger to max level: {}", log::max_level());
            ()
        }
        Err(e) => {
            eprintln!("Error initializing Rust Logger: {}", e);
        }
    }
}
