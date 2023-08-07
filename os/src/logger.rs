//! Record different levels of log.

use log::{ Log, Record, Level, LevelFilter, Metadata };

struct Logger;

impl Log for Logger {
    fn enabled(&self, _meta: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let color = match record.level() {
            Level::Error => 31,
            Level::Warn => 93,
            Level::Info => 34,
            Level::Debug => 32,
            Level::Trace => 90,
        };

        println!("\x1b[{}m{}\x1b[0m", color, record.args());
    }

    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: Logger = Logger;

    let _ = match option_env!("LOG") {
        Some("ERROR") => log::set_logger(&LOGGER).map(|()| { log::set_max_level(LevelFilter::Error); }),
        Some("WARN") => log::set_logger(&LOGGER).map(|()| { log::set_max_level(LevelFilter::Warn); }),
        Some("INFO") => log::set_logger(&LOGGER).map(|()| { log::set_max_level(LevelFilter::Info); }),
        Some("DEBUG") => log::set_logger(&LOGGER).map(|()| { log::set_max_level(LevelFilter::Debug); }),
        Some("TRACE") => log::set_logger(&LOGGER).map(|()| { log::set_max_level(LevelFilter::Trace); }),
        _ => log::set_logger(&LOGGER).map(|()| { log::set_max_level(LevelFilter::Off); }),
    };
}

