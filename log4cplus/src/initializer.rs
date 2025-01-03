use super::ffi::log4cplus::Initializer as FFI;
use autocxx::WithinUniquePtr;
use cxx::UniquePtr;

pub struct Initializer(UniquePtr<FFI>);

impl Initializer {
    pub fn new() -> Self {
        Self(FFI::new().within_unique_ptr())
    }
}

impl Default for Initializer {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<FFI> for Initializer {
    fn as_ref(&self) -> &FFI {
        &self.0
    }
}
