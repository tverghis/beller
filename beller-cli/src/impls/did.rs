/// Prints the recommended DID credentials for the given `access_token`.
pub fn get_recommended_credentials(pds: &str, access_token: &str) {
    let creds = super::api::did::get_recommended_credentials(access_token, pds);
    println!("{}", serde_json::to_string_pretty(&creds).unwrap());
}
