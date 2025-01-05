#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

#[macro_export]
macro_rules! log {
    ($level:expr, $logger:expr, $($msg:tt)*) => {{
        $logger.log(log4cplus::LogLevel::from($level), std::format!($($msg)*), std::file!(), std::line!(), log4cplus::function!());
    }};
    ($level:expr, $($msg:tt)*) => {{
        log4cplus::log!($level, log4cplus::Logger::default(), $($msg)*);
    }};
}

#[macro_export]
macro_rules! trace {
    ($logger:expr, $($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Trace, $logger, $($msg)*);
    };
    ($($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Trace, $($msg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Debug, $logger, $($msg)*);
    };
    ($($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Debug, $($msg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Info, $logger, $($msg)*);
    };
    ($($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Info, $($msg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Warn, $logger, $($msg)*);
    };
    ($($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Warn, $($msg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Error, $logger, $($msg)*);
    };
    ($($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Error, $($msg)*);
    };
}

#[macro_export]
macro_rules! fatal {
    ($logger:expr, $($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Fatal, $logger, $($msg)*);
    };
    ($($msg:tt)*) => {
        log4cplus::log!(log4cplus::LogLevel::Fatal, $($msg)*);
    };
}

#[macro_export]
macro_rules! assert {
    ($logger:expr, $condition:expr) => {
        $logger.assertion(
            $condition,
            format!("failed condition: {}", stringify!($condition)),
        );
    };
    ($condition:expr) => {
        log4cplus::assert!(log4cplus::Logger::default(), $condition);
    };
}
