use log::warn;
use log4cplus::{BasicConfigurator, Initializer};
use log_log4cplus::Log4CPlusLogger;

/// Rust version of <https://github.com/log4cplus/log4cplus/wiki/Code-Examples#hello-world>, using `log` macros.`
fn main() {
    let _initializer = Initializer::default();

    let mut configurator = BasicConfigurator::default();
    configurator.configure();

    Log4CPlusLogger::default().try_init().unwrap();

    warn!("Hello, world!");
}
