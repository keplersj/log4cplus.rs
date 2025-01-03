use super::ffi2::{basic_configurator_new, BasicConfigurator as FFI};

pub struct BasicConfigurator(cxx::UniquePtr<FFI>);

impl BasicConfigurator {
    pub fn new() -> Self {
        Self(basic_configurator_new())
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

impl AsRef<FFI> for BasicConfigurator {
    fn as_ref(&self) -> &FFI {
        &self.0
    }
}
