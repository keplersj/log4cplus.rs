use super::ffi;
use std::fmt::{self, Display};

pub struct TString(cxx::UniquePtr<ffi::TString>);

impl TString {
    pub fn new(s: impl AsRef<[u8]>) -> Self {
        s.into()
    }
}

impl Default for TString {
    fn default() -> Self {
        // This mirrors the default behavior of Rust String
        Self::new("")
    }
}

impl<T> From<T> for TString
where
    T: AsRef<[u8]>,
{
    fn from(s: T) -> Self {
        // Create a CxxString from the input, then convert it to a TString
        // This codepath doesn't require `unsafe`
        cxx::let_cxx_string!(cxxstr = s);
        let ptr = ffi::string_to_tstring(&cxxstr);

        Self(ptr)
    }
}

impl AsRef<ffi::TString> for TString {
    fn as_ref(&self) -> &ffi::TString {
        &self.0
    }
}

impl Display for TString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cxxstr = ffi::tstring_to_string(self.as_ref());

        f.write_str(cxxstr.to_str().map_err(|_| fmt::Error)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tstring_str() {
        let tstring = TString::new("hello");
        assert_eq!(tstring.to_string(), "hello");
    }

    #[test]
    fn test_tstring_string() {
        let tstring = TString::new(String::from("hello"));
        assert_eq!(tstring.to_string(), "hello");
    }

    #[test]
    fn test_tstring_fmt() {
        let injectable = "world";
        let tstring = TString::new(format!("Hello, {injectable}!"));
        assert_eq!(tstring.to_string(), "Hello, world!");
    }

    #[test]
    fn test_tstring_default() {
        let tstring = TString::default();
        assert_eq!(tstring.to_string(), "");
    }

    // Currently fails if `proptest` uses characters like `𞄀` (U+1E100)
    // proptest! {
    //     #[test]
    //     fn test_tstring_proptest(s: String) {
    //         let tstring = TString::new(s.clone());
    //         assert_eq!(tstring.to_string(), s);
    //     }
    // }
}
