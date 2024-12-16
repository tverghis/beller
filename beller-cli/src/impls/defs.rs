use std::ffi::OsString;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PdsUrl(String);

impl From<OsString> for PdsUrl {
    fn from(value: OsString) -> Self {
        let value_str = value.to_string_lossy().to_string();

        Self(value_str)
    }
}

impl<'a> From<&'a PdsUrl> for &'a str {
    fn from(value: &'a PdsUrl) -> &'a str {
        &value.0
    }
}
