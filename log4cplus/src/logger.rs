use super::ffi;
use crate::{LogLevel, TString};
use std::ffi::CString;

pub struct Logger(cxx::UniquePtr<ffi::Logger>);

impl AsRef<ffi::Logger> for Logger {
    fn as_ref(&self) -> &ffi::Logger {
        &self.0
    }
}

impl Logger {
    pub fn new(name: impl Into<TString>) -> Self {
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
        file: impl Into<Vec<u8>>,
        line: u32,
        function: impl Into<Vec<u8>>,
    ) {
        let file = CString::new(file).unwrap();
        let function = CString::new(function).unwrap();
        let tstr = message.into();
        unsafe {
            self.as_ref().log(
                log_level.into(),
                tstr.as_ref(),
                file.as_ptr(),
                line as i32,
                function.as_ptr(),
            )
        };
    }
}
