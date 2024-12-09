use beller_lib::XRPC;

use super::api::did::DidCreds;

/// Returns the DID associated with the session for the given `access_token`.
pub fn get_for_session(access_token: &str, pds: &str) {
    let did = match beller_lib::GetSession::new(access_token.to_string()).apply(pds) {
        Ok(session) => session.did.to_string(),
        Err(e) => {
            eprintln!(
                "Failed to perform `{}`: `{e}`",
                beller_lib::GetSession::NSID
            );
            std::process::exit(1);
        }
    };

    super::repo::describe(&did, pds);
}

/// Fetches the recommended DID credentials for the given `access_token`.
pub fn get_recommended_credentials(access_token: &str, pds: &str) -> DidCreds {
    super::api::did::get_recommended_credentials(access_token, pds)
}
