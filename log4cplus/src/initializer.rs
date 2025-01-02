use super::ffi;

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
