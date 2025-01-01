use log4cplus::{BasicConfigurator, Initializer, LogLevel, Logger};
use std::ffi::CString;

/// Rust version of <https://github.com/log4cplus/log4cplus/wiki/Code-Examples#hello-world>
fn main() {
    let _initializer = Initializer::new();

    let mut configurator = BasicConfigurator::new();
    configurator.configure();

    let logger = Logger::get_instance("hello_world");
    logger.log(
        LogLevel::All,
        "hello all",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::Debug,
        "hello debug",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::Error,
        "hello error",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::Error,
        "hello error",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::Fatal,
        "hello fatal",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::Info,
        "hello info",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::NotSet,
        "hello notset",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::Off,
        "hello off",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::Trace,
        "hello trace",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
    logger.log(
        LogLevel::Warn,
        "hello warn",
        &CString::new(file!()).unwrap(),
        line!(),
        &CString::new("main").unwrap(),
    );
}
