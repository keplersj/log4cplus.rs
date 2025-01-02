use super::ffi;

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
