use beller_lib::XRPC;

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

pub type DidCreds = <beller_lib::GetRecommendedDidCredentials as XRPC>::Return;

/// Fetches the recommended DID credentials for the given `access_token`.
pub fn get_recommended_credentials(access_token: &str, pds: &str) -> DidCreds {
    match beller_lib::GetRecommendedDidCredentials::new(access_token.into()).apply(pds) {
        Ok(did_creds) => did_creds,
        Err(e) => {
            eprintln!(
                "Failed to perform `{}`: {e}",
                beller_lib::GetRecommendedDidCredentials::NSID
            );
            std::process::exit(1);
        }
    }
}
