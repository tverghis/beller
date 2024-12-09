use beller_lib::XRPC;

pub type DidCreds = <beller_lib::GetRecommendedDidCredentials as XRPC>::Return;

/// Fetches the recommended DID credentials for the given `access_token`.
pub fn get_recommended_credentials(access_token: &str, pds: &str) -> DidCreds {
    beller_lib::GetRecommendedDidCredentials::new(access_token.into())
        .apply(pds)
        .expect("could not get recommended DID credentials")
}
