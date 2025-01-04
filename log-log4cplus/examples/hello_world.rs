use log::warn;
use log4cplus::{BasicConfigurator, Initializer, Logger};
use log_log4cplus::Log4CPlusLogger;

/// Rust version of <https://github.com/log4cplus/log4cplus/wiki/Code-Examples#hello-world>, using `log` macros.`
fn main() {
    let _initializer = Initializer::new();

    let mut configurator = BasicConfigurator::new();
    configurator.configure();

    let logger = Logger::get_instance("main");
    Log4CPlusLogger::from(logger).try_init().unwrap();

    warn!("Hello, world!");
}
