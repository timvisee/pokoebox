extern crate log;

use self::log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata, SetLoggerError};

/// Application logger.
pub struct Logger;

impl Logger {
    /// Initialize the logger globally.
    pub fn init() -> Result<(), SetLoggerError> {
        // Create a new logger
        let logger = Logger {};

        // Set the logger
        self::log::set_logger(|max_log_level| {
            // Use the proper logging level
            max_log_level.set(LogLevelFilter::Info);
            Box::new(logger)
        })
    }
}

impl self::log::Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}