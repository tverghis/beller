use beller_lib::{session, XRPC};

use crate::cli::Credentials;

impl From<&Credentials> for session::Create {
    fn from(args: &Credentials) -> Self {
        Self {
            identifier: args.identifier.clone(),
            password: args.password.clone(),
        }
    }
}

/// Creates a new session on the PDS.
pub fn create(args: &Credentials, pds: &str) -> <session::Create as XRPC>::Return {
    session::Create::from(args)
        .apply(pds)
        .expect("could not create new session")
}

/// Fetches details about the current session from the PDS.
pub fn get(access_token: &str, pds: &str) -> <session::Get as XRPC>::Return {
    session::Get::new(access_token.to_string())
        .apply(pds)
        .expect("could not get session")
}
