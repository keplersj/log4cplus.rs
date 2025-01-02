#[cxx::bridge(namespace = "log4cplus")]
mod ffi {
    unsafe extern "C++" {
        include!("log4cplus/tstring.h");
        include!("log4cplus/logger.h");
        include!("log4cplus/configurator.h");
        include!("log4cplus/initializer.h");
        include!("shim.hpp");

        type Initializer;
        #[rust_name = "initializer_new"]
        fn construct_unique() -> UniquePtr<Initializer>;

        type BasicConfigurator;
        #[rust_name = "basic_configurator_new"]
        fn construct_unique() -> UniquePtr<BasicConfigurator>;
        fn configure(self: Pin<&mut BasicConfigurator>);

        #[cxx_name = "tstring"]
        type TString;
        fn string_to_tstring(str: &CxxString) -> UniquePtr<TString>;
        // A valid conversion, could be helpful, not currently needed.
        // It's use of c_char requiring `unsafe` is less than ideal
        #[allow(dead_code)]
        unsafe fn cstr_to_tstring(str: *const c_char) -> UniquePtr<TString>;
        fn tstring_to_string(str: &TString) -> UniquePtr<CxxString>;

        type Logger;
        fn Logger_getInstance(name: &TString) -> UniquePtr<Logger>;
        // The function signature's use of `*const c_char` requires the `unsafe`
        unsafe fn log(
            self: &Logger,
            log_level: i32,
            message: &TString,
            file: *const c_char,
            line: i32,
            function: *const c_char,
        );
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
