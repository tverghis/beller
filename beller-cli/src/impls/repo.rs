use super::defs::PdsUrl;

/// Prints details about the repository for `did`.
pub fn describe_did(pds: &PdsUrl, did: &str) {
    let res = super::api::repo::describe(pds, did);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

/// Prints details about the repository for the `did` in the session associated with the `access_token`.
pub fn describe_session(pds: &PdsUrl, access_token: &str) {
    let session = super::api::session::get(pds, access_token);
    describe_did(pds, &session.did);
}
