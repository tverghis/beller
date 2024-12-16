use beller_lib::{identity, XRPC};

use crate::impls::defs::PdsUrl;

pub type RecommendedCredentials = <identity::GetRecommendedDidCredentials as XRPC>::Return;

/// Fetches the recommended DID credentials for the given `access_token`.
pub fn get_recommended_credentials(pds: &PdsUrl, access_token: &str) -> RecommendedCredentials {
    identity::GetRecommendedDidCredentials::new(access_token.into())
        .apply(pds.into())
        .expect("could not get recommended DID credentials")
}
