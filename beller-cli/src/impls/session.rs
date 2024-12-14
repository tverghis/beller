use super::api::session;
use crate::cli::Credentials;

/// Creates a new session on the PDS.
pub fn create(pds: &str, args: &Credentials) {
    let res = session::create(args, pds);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

/// Prints details about the current session.
pub fn get(pds: &str, access_token: &str) {
    let res = session::get(access_token, pds);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
