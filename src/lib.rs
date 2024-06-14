use std::io::Write;

use log::Log;

pub use log::{debug, error, info, trace, warn, LevelFilter};

pub struct Logger {}

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let prefix = match record.level() {
            log::Level::Error => "\x1b[1;31m[e]\x1b[0m",
            log::Level::Warn => "\x1b[1;33m[w]\x1b[0m",
            log::Level::Info => "\x1b[1;32m[i]\x1b[0m",
            log::Level::Debug => "\x1b[1;36m[d]\x1b[0m",
            log::Level::Trace => "\x1b[1m[t]\x1b[0m",
        };

        let _ = std::io::stderr().write_fmt(format_args!("{prefix} {}\n", record.args()));
    }

    fn flush(&self) {
        let _ = std::io::stderr().flush();
    }
}

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&Logger {})?;
    log::set_max_level(log::LevelFilter::Trace);

    Ok(())
}

pub fn set_log_level(filter: log::LevelFilter) {
    log::set_max_level(filter);
}
