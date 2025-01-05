use log4cplus::{
    assert, debug, error, fatal, function, info, trace, warn, BasicConfigurator, Initializer,
    Logger,
};

/// Demonstation of all of the logger macros provided by this crate
fn main() {
    let _initializer = Initializer::new();

    let mut configurator = BasicConfigurator::new();
    configurator.configure();

    let logger = Logger::get_instance("main");

    // Demonstation of `fmt`-style interpolaton
    debug!(logger, "hello {} @ {}:{}", function!(), file!(), line!());

    debug!(logger, "hello debug");
    error!(logger, "hello error");
    fatal!(logger, "hello fatal");
    info!(logger, "hello info");
    trace!(logger, "hello trace");
    warn!(logger, "hello warn");

    // This should not log
    assert!(logger, true);

    // This should log a FATAL message
    assert!(logger, 1 == 2);
}
