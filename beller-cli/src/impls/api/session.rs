use beller_lib::XRPC;

use crate::cli::Credentials;

/// Creates a new session on the PDS.
pub fn create(args: &Credentials, pds: &str) -> <beller_lib::CreateSession as XRPC>::Return {
    beller_lib::CreateSession::from(args)
        .apply(pds)
        .expect("could not create new session")
}

/// Fetches details about the current session from the PDS.
pub fn get(access_token: &str, pds: &str) -> <beller_lib::GetSession as XRPC>::Return {
    beller_lib::GetSession::new(access_token.to_string())
        .apply(pds)
        .expect("could not get session")
}
