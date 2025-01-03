use log4cplus::{function, BasicConfigurator, Initializer, LogLevel, Logger};

/// Rust version of <https://github.com/log4cplus/log4cplus/wiki/Code-Examples#hello-world>
fn main() {
    let _initializer = Initializer::new();

    let mut configurator = BasicConfigurator::new();
    configurator.configure();

    let logger = Logger::get_instance("hello_world");
    logger.log(
        LogLevel::Debug,
        format!("hello {} @ {}:{}", function!(), file!(), line!()),
        file!(),
        line!(),
        function!(),
    );
    logger.log(LogLevel::All, "hello all", file!(), line!(), function!());
    logger.log(
        LogLevel::Debug,
        "hello debug",
        file!(),
        line!(),
        function!(),
    );
    logger.log(
        LogLevel::Error,
        "hello error",
        file!(),
        line!(),
        function!(),
    );
    logger.log(
        LogLevel::Fatal,
        "hello fatal",
        file!(),
        line!(),
        function!(),
    );
    logger.log(LogLevel::Info, "hello info", file!(), line!(), function!());
    logger.log(
        LogLevel::NotSet,
        "hello notset",
        file!(),
        line!(),
        function!(),
    );
    logger.log(LogLevel::Off, "hello off", file!(), line!(), function!());
    logger.log(
        LogLevel::Trace,
        "hello trace",
        file!(),
        line!(),
        function!(),
    );
    logger.log(LogLevel::Warn, "hello warn", file!(), line!(), function!());
}
