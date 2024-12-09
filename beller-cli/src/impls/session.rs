use super::api::session;
use crate::cli::Credentials;

/// Creates a new session on the PDS.
pub fn create(args: &Credentials, pds: &str) {
    let res = session::create(args, pds);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

/// Prints details about the current session.
pub fn get(access_token: &str, pds: &str) {
    let res = session::get(access_token, pds);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
