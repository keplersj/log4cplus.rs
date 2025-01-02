pub mod tstring;
pub use tstring::TString;

use std::ffi::CStr;

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

pub struct Initializer(cxx::UniquePtr<ffi::Initializer>);

impl Initializer {
    pub fn new() -> Self {
        Self(ffi::initializer_new())
    }
}

impl Default for Initializer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BasicConfigurator(cxx::UniquePtr<ffi::BasicConfigurator>);

impl BasicConfigurator {
    pub fn new() -> Self {
        Self(ffi::basic_configurator_new())
    }

    pub fn configure(&mut self) {
        self.0.pin_mut().configure();
    }
}

impl Default for BasicConfigurator {
    fn default() -> Self {
        Self::new()
    }
}

/// Equivalent to log4cplus::LogLevel: <https://log4cplus.github.io/log4cplus/docs/log4cplus-2.1.0/doxygen/namespacelog4cplus.html#abd332cc8c98fefcbbdcf57b6b3867de9>
pub enum LogLevel {
    Off,
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    All,
    NotSet,
}

impl From<LogLevel> for i32 {
    fn from(log_level: LogLevel) -> i32 {
        match log_level {
            LogLevel::Off => 60000,
            LogLevel::Fatal => 50000,
            LogLevel::Error => 40000,
            LogLevel::Warn => 30000,
            LogLevel::Info => 20000,
            LogLevel::Debug => 10000,
            LogLevel::Trace => 0,
            LogLevel::All => 0,
            LogLevel::NotSet => -1,
        }
    }
}

pub struct Logger(cxx::UniquePtr<ffi::Logger>);

impl Logger {
    pub fn new(name: &str) -> Self {
        Self::get_instance(name)
    }

    pub fn get_instance(name: impl Into<TString>) -> Self {
        let tstr: TString = name.into();
        Self(ffi::Logger_getInstance(tstr.as_ref()))
    }

    pub fn log(
        &self,
        log_level: LogLevel,
        message: impl Into<TString>,
        file: &CStr,
        line: u32,
        function: &CStr,
    ) {
        let file = file.as_ptr();
        let function = function.as_ptr();
        let tstr = message.into();
        unsafe {
            self.0
                .log(log_level.into(), tstr.as_ref(), file, line as i32, function)
        };
    }
}
