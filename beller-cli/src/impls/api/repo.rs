use beller_lib::{repo, XRPC};

use crate::impls::defs::PdsUrl;

/// Fetches details about the repository for `did`.
pub fn describe(pds: &PdsUrl, did: &str) -> <repo::Describe as XRPC>::Return {
    repo::Describe::new(did.to_string())
        .apply(pds.into())
        .expect("could not describe repo")
}
