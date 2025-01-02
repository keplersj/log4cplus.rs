/// Equivalent to log4cplus::LogLevel: <https://log4cplus.github.io/log4cplus/docs/log4cplus-2.1.0/doxygen/namespacelog4cplus.html#abd332cc8c98fefcbbdcf57b6b3867de9>
pub enum LogLevel {
    Off,
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    All,
    NotSet,
}

impl From<LogLevel> for i32 {
    fn from(log_level: LogLevel) -> i32 {
        match log_level {
            LogLevel::Off => 60000,
            LogLevel::Fatal => 50000,
            LogLevel::Error => 40000,
            LogLevel::Warn => 30000,
            LogLevel::Info => 20000,
            LogLevel::Debug => 10000,
            LogLevel::Trace => 0,
            LogLevel::All => 0,
            LogLevel::NotSet => -1,
        }
    }
}
