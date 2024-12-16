use std::ffi::OsString;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PdsUrl(String);

impl Default for PdsUrl {
    fn default() -> Self {
        Self(String::from("https://bsky.social"))
    }
}

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
