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

use fern;
use fern::colors::{Color, ColoredLevelConfig};
use log::{LevelFilter};
use chrono;

use std::io;

fn setup_logger(verbosity: u64) -> Result<(), fern::InitError> {
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

pub fn init() -> () {
    /*! Initialisation of [Log Crate](https://crates.io/crates/log) and [Fern Crate](https://docs.rs/fern/0.5.5/fern/) */
    /*! to output to both stdout and to a log file with choice of logging level macros from highest priority to lowest: */
    /*! `error!`, `warn!`, `info!`, `debug!` and `trace!`. */
    /*! [Compile time filters](https://docs.rs/log/0.4.1/log/#compile-time-filters) are configured in Cargo.toml */

    let verbosity: u64 = 4;

    match setup_logger(verbosity) {
        Ok(res) => { info!("Success initializing Rust Logger to verbosity level: {}", verbosity); () }
        Err(e) => { error!("Error initializing Rust Logger: {}", e); }
    }
}
