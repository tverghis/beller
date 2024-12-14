/// Prints details about the repository for `did`.
pub fn describe_did(pds: &str, did: &str) {
    let res = super::api::repo::describe(did, pds);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

/// Prints details about the repository for the `did` in the session associated with the `access_token`.
pub fn describe_session(pds: &str, access_token: &str) {
    let session = super::api::session::get(access_token, pds);
    describe_did(&session.did, pds);
}
