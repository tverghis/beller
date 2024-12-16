use super::defs::PdsUrl;

/// Prints the recommended DID credentials for the given `access_token`.
pub fn get_recommended_credentials(pds: &PdsUrl, access_token: &str) {
    let creds = super::api::did::get_recommended_credentials(pds, access_token);
    println!("{}", serde_json::to_string_pretty(&creds).unwrap());
}
