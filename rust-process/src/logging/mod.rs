use log::{Level, LevelFilter, Metadata, Record};
use std::sync::Once;
use time::OffsetDateTime;

static INIT: Once = Once::new();

/// Custom logger implementation with detailed formatting
pub struct SecurityLogger;

impl log::Log for SecurityLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = OffsetDateTime::now_utc();
            let timestamp = now.format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| String::from("timestamp-error"));

            let level_color = match record.level() {
                Level::Error => "\x1b[31m", // Red
                Level::Warn => "\x1b[33m",  // Yellow
                Level::Info => "\x1b[32m",  // Green
                Level::Debug => "\x1b[36m", // Cyan
                Level::Trace => "\x1b[90m", // Bright Black
            };

            eprintln!(
                "{}{} [{}] {} - {}\x1b[0m",
                level_color,
                timestamp,
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

/// Initialize the logging system
pub fn init(level: LevelFilter) {
    INIT.call_once(|| {
        log::set_boxed_logger(Box::new(SecurityLogger))
            .map(|()| log::set_max_level(level))
            .expect("Failed to initialize logger");
    });
}

/// Initialize test logging with appropriate level
pub fn init_test_logging() {
    let level = match std::env::var("RUST_LOG") {
        Ok(level) => match level.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Debug,
        },
        Err(_) => LevelFilter::Debug,
    };
    init(level);
}
