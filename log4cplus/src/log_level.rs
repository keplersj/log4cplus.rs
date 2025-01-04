use autocxx::c_int;

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
    Unknown(i32),
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
            LogLevel::Unknown(level) => level,
        }
    }
}

impl From<i32> for LogLevel {
    fn from(level: i32) -> LogLevel {
        match level {
            60000 => LogLevel::Off,
            50000 => LogLevel::Fatal,
            40000 => LogLevel::Error,
            30000 => LogLevel::Warn,
            20000 => LogLevel::Info,
            10000 => LogLevel::Debug,
            0 => LogLevel::Trace,
            -1 => LogLevel::NotSet,
            level => LogLevel::Unknown(level),
        }
    }
}

impl From<LogLevel> for c_int {
    fn from(log_level: LogLevel) -> c_int {
        c_int(i32::from(log_level))
    }
}

impl From<c_int> for LogLevel {
    fn from(level: c_int) -> LogLevel {
        level.0.into()
    }
}
