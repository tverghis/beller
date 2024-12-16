use super::{api::session, defs::PdsUrl};
use crate::cli::Credentials;

/// Creates a new session on the PDS.
pub fn create(pds: &PdsUrl, args: &Credentials) {
    let res = session::create(pds, args);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

/// Prints details about the current session.
pub fn get(pds: &PdsUrl, access_token: &str) {
    let res = session::get(pds, access_token);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
