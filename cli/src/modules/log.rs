use fern;
use fern::colors::{Color, ColoredLevelConfig};
use log::{LevelFilter};
use chrono;

use std::io;

fn setup_logger(verbosity: u32) -> Result<(), fern::InitError> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);

    let colors_level = colors_line.clone()
        .info(Color::Green);

    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => { base_config
            .level(LevelFilter::Error)
            .level_for("pretty_colored", LevelFilter::Error)
        }
        1 => { base_config
            .level(LevelFilter::Warn)
            .level_for("pretty_colored", LevelFilter::Warn)
        },
        2 => { base_config
            .level(LevelFilter::Info)
            .level_for("pretty_colored", LevelFilter::Info)
        },
        3 => { base_config
            .level(LevelFilter::Debug)
            .level_for("pretty_colored", LevelFilter::Debug)
        },
        _ => { base_config
            .level(LevelFilter::Trace)
            .level_for("pretty_colored", LevelFilter::Trace)
        },
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
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!("\x1B[{}m", colors_line.get_color(&record.level()).to_fg_str()),
                date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message
            ))
        })
        .chain(io::stdout());

    base_config.chain(file_config).chain(stdout_config).apply()?;

    Ok(())
}

pub fn init_logger(pattern: &str) -> () {
    /*! Initialisation of [Log Crate](https://crates.io/crates/log) and [Fern Crate](https://docs.rs/fern/0.5.5/fern/) */
    /*! to output to both stdout and to a log file with choice of logging level macros from highest priority to lowest: */
    /*! `error!`, `warn!`, `info!`, `debug!` and `trace!`. */
    /*! [Compile time filters](https://docs.rs/log/0.4.1/log/#compile-time-filters) are configured in Cargo.toml */

    let verbosity: u32 = pattern.parse::<u32>().unwrap();

    match setup_logger(verbosity) {
        Ok(_) => { info!("Success initializing Rust Logger to verbosity level: {:?}", verbosity); () }
        Err(e) => { error!("Error initializing Rust Logger: {}", e); }
    }
}
