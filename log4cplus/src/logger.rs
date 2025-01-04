use super::ffi::log4cplus::Logger as FFI;
use crate::{LogLevel, TString};
use autocxx::WithinUniquePtr;
use std::ffi::CString;

pub struct Logger(cxx::UniquePtr<FFI>);

impl AsRef<FFI> for Logger {
    fn as_ref(&self) -> &FFI {
        &self.0
    }
}

impl Logger {
    pub fn new(name: impl Into<TString>) -> Self {
        Self::get_instance(name)
    }

    pub fn get_instance(name: impl Into<TString>) -> Self {
        let tstr: TString = name.into();
        Self(FFI::getInstance(tstr.as_ref()).within_unique_ptr())
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

    pub fn is_enabled_for(&self, log_level: LogLevel) -> bool {
        self.as_ref().isEnabledFor(autocxx::c_int(log_level.into()))
    }
}
