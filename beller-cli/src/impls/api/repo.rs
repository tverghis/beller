use beller_lib::XRPC;

/// Fetches details about the repository for `did`.
pub fn describe(did: &str, pds: &str) -> <beller_lib::DescribeRepo as XRPC>::Return {
    beller_lib::DescribeRepo::new(did.to_string())
        .apply(pds)
        .expect("could not describe repo")
}
