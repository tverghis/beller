use super::api::did::DidCreds;

/// Fetches the recommended DID credentials for the given `access_token`.
///
/// TODO: this shouldn't return anything; callers should directly invoke the API layer.
pub fn get_recommended_credentials(access_token: &str, pds: &str) -> DidCreds {
    super::api::did::get_recommended_credentials(access_token, pds)
}
