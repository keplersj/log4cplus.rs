use log4cplus::{warn, BasicConfigurator, Initializer, Logger};

/// Rust version of <https://github.com/log4cplus/log4cplus/wiki/Code-Examples#hello-world>
///
/// This example is expected to match 1:1 with the following C++ code:
///
/// ```cpp
/// #include <log4cplus/logger.h>
/// #include <log4cplus/loggingmacros.h>
/// #include <log4cplus/configurator.h>
/// #include <log4cplus/initializer.h>
///
/// int
/// main()
/// {
///     // Initialization and deinitialization.
///     log4cplus::Initializer initializer;
///
///     log4cplus::BasicConfigurator config;
///     config.configure();
///
///     log4cplus::Logger logger = log4cplus::Logger::getInstance(
///         LOG4CPLUS_TEXT("main"));
///     LOG4CPLUS_WARN(logger, LOG4CPLUS_TEXT("Hello, World!"));
///     return 0;
/// }
/// ```
fn main() {
    let _initializer = Initializer::new();

    let mut configurator = BasicConfigurator::new();
    configurator.configure();

    let logger = Logger::get_instance("main");
    warn!(logger, "Hello, World!");
}
