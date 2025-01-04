use crate::{
    ffi::log4cplus::{tstring_to_string, Logger as FFI},
    LogLevel, TString,
};
use autocxx::WithinUniquePtr;
use cxx::UniquePtr;
use std::ffi::CString;

pub struct Logger(UniquePtr<FFI>);

impl AsRef<FFI> for Logger {
    fn as_ref(&self) -> &FFI {
        &self.0
    }
}

impl Logger {
    pub fn exists(name: impl Into<TString>) -> bool {
        let tstr: TString = name.into();
        FFI::exists(tstr.as_ref())
    }

    // TODO: This function currently causes a seg fault when the Vec is Dropped.
    pub unsafe fn current_loggers() -> Vec<Self> {
        FFI::getCurrentLoggers()
            .pin_mut()
            .into_iter()
            .map(|logger| unsafe { Self(UniquePtr::from_raw(logger.get_unchecked_mut())) })
            .collect()
    }

    pub fn new(name: impl Into<TString>) -> Self {
        Self::get_instance(name)
    }

    pub fn get_instance(name: impl Into<TString>) -> Self {
        let tstr: TString = name.into();
        FFI::getInstance(tstr.as_ref()).within_unique_ptr().into()
    }

    pub fn root() -> Self {
        FFI::getRoot().within_unique_ptr().into()
    }

    pub unsafe fn shutdown() {
        FFI::shutdown()
    }

    pub fn assertion(&self, value: bool, message: impl Into<TString>) {
        let tstr: TString = message.into();
        self.as_ref().assertion(value, tstr.as_ref());
    }

    pub fn close_nested_appenders(&self) {
        self.as_ref().closeNestedAppenders()
    }

    pub fn is_enabled_for(&self, log_level: LogLevel) -> bool {
        self.as_ref().isEnabledFor(autocxx::c_int(log_level.into()))
    }

    pub fn log(
        &self,
        log_level: LogLevel,
        message: impl Into<TString>,
        file: impl Into<Vec<u8>>,
        line: u32,
        function: impl Into<Vec<u8>>,
    ) {
        let file = CString::new(file).unwrap();
        let function = CString::new(function).unwrap();
        let tstr = message.into();
        unsafe {
            self.as_ref().log(
                autocxx::c_int(log_level.into()),
                tstr.as_ref(),
                file.as_ptr(),
                autocxx::c_int(line as i32),
                function.as_ptr(),
            )
        };
    }

    pub fn forced_log(
        &self,
        log_level: LogLevel,
        message: impl Into<TString>,
        file: impl Into<Vec<u8>>,
        line: u32,
        function: impl Into<Vec<u8>>,
    ) {
        let file = CString::new(file).unwrap();
        let function = CString::new(function).unwrap();
        let tstr = message.into();
        unsafe {
            self.as_ref().forcedLog(
                autocxx::c_int(log_level.into()),
                tstr.as_ref(),
                file.as_ptr(),
                autocxx::c_int(line as i32),
                function.as_ptr(),
            )
        };
    }

    pub fn chained_log_level(&self) -> LogLevel {
        self.as_ref().getChainedLogLevel().into()
    }

    pub fn log_level(&self) -> LogLevel {
        self.as_ref().getLogLevel().into()
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) {
        self.0.pin_mut().setLogLevel(log_level.into());
    }

    pub fn name(&self) -> String {
        tstring_to_string(self.as_ref().getName()).to_string()
    }

    pub fn additivity(&self) -> bool {
        self.as_ref().getAdditivity()
    }

    pub fn set_additivity(&mut self, additivity: bool) {
        self.0.pin_mut().setAdditivity(additivity);
    }
}

impl From<UniquePtr<FFI>> for Logger {
    fn from(logger: UniquePtr<FFI>) -> Self {
        Self(logger)
    }
}

impl From<Logger> for UniquePtr<FFI> {
    fn from(logger: Logger) -> Self {
        logger.0
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::root()
    }
}
