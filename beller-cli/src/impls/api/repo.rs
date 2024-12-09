use beller_lib::{repo, XRPC};

/// Fetches details about the repository for `did`.
pub fn describe(did: &str, pds: &str) -> <repo::Describe as XRPC>::Return {
    repo::Describe::new(did.to_string())
        .apply(pds)
        .expect("could not describe repo")
}
