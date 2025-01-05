use log4cplus::{
    debug, error, fatal, function, info, trace, warn, BasicConfigurator, Initializer, LogLevel,
    Logger,
};

/// Rust version of <https://github.com/log4cplus/log4cplus/wiki/Code-Examples#hello-world>
fn main() {
    let _initializer = Initializer::new();

    let mut configurator = BasicConfigurator::new();
    configurator.configure();

    let logger = Logger::get_instance("hello_world");
    debug!(logger, "hello {} @ {}:{}", function!(), file!(), line!());
    logger.log(LogLevel::All, "hello all", file!(), line!(), function!());
    debug!(logger, "hello debug");
    error!(logger, "hello error");
    fatal!(logger, "hello fatal");
    info!(logger, "hello info");
    logger.log(
        LogLevel::NotSet,
        "hello notset",
        file!(),
        line!(),
        function!(),
    );
    logger.log(LogLevel::Off, "hello off", file!(), line!(), function!());
    trace!(logger, "hello trace");
    warn!(logger, "hello warn");
}
