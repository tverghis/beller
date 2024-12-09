use beller_lib::{identity, XRPC};

pub type RecommendedCredentials = <identity::GetRecommendedDidCredentials as XRPC>::Return;

/// Fetches the recommended DID credentials for the given `access_token`.
pub fn get_recommended_credentials(access_token: &str, pds: &str) -> RecommendedCredentials {
    identity::GetRecommendedDidCredentials::new(access_token.into())
        .apply(pds)
        .expect("could not get recommended DID credentials")
}
