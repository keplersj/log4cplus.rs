use log::{Level, LevelFilter, Log, Metadata, Record};
use log4cplus::Logger;

pub struct Log4CPlusLogger {
    logger: Logger,
}

impl From<Logger> for Log4CPlusLogger {
    fn from(logger: Logger) -> Self {
        Log4CPlusLogger { logger }
    }
}

fn level_to_loglevel(level: Level) -> log4cplus::LogLevel {
    match level {
        Level::Error => log4cplus::LogLevel::Error,
        Level::Warn => log4cplus::LogLevel::Warn,
        Level::Info => log4cplus::LogLevel::Info,
        Level::Debug => log4cplus::LogLevel::Debug,
        Level::Trace => log4cplus::LogLevel::Trace,
    }
}

impl Log for Log4CPlusLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.logger
            .is_enabled_for(level_to_loglevel(metadata.level()))
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.logger.log(
                level_to_loglevel(record.level()),
                record.args().to_string(),
                record.file().unwrap_or_default(),
                record.line().unwrap_or_default(),
                format!(
                    "{}::{}",
                    record.module_path().unwrap_or_default(),
                    record.target()
                ),
            );
        }
    }

    fn flush(&self) {}
}

impl Log4CPlusLogger {
    pub fn try_init(self) -> Result<(), log::SetLoggerError> {
        log::set_max_level(LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self))
    }
}
