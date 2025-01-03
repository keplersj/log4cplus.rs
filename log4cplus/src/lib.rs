use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "log4cplus/logger.h"
    #include "log4cplus/configurator.h"
    #include "log4cplus/initializer.h"
    #include "shim.hpp"
    safety!(unsafe)
    generate_pod!("log4cplus::LogLevel")
    generate!("log4cplus::tstring")

    generate!("log4cplus::Initializer")
    generate!("log4cplus::BasicConfigurator")
    generate!("log4cplus::Logger")

    // shim.h
    generate!("log4cplus::string_to_tstring")
    generate!("log4cplus::tstring_to_string")
}

#[cxx::bridge(namespace = "log4cplus")]
mod ffi2 {
    unsafe extern "C++" {
        include!("log4cplus/configurator.h");
        include!("shim.hpp");

        type BasicConfigurator = super::ffi::log4cplus::BasicConfigurator;
        #[rust_name = "basic_configurator_new"]
        fn construct_unique() -> UniquePtr<BasicConfigurator>;
        fn configure(self: Pin<&mut BasicConfigurator>);
    }
}

mod tstring;
pub use tstring::TString;

mod initializer;
pub use initializer::Initializer;

mod basic_configurator;
pub use basic_configurator::BasicConfigurator;

mod log_level;
pub use log_level::LogLevel;

mod logger;
pub use logger::Logger;
