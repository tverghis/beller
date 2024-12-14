use super::api::plc;

/// Requests an operation signing token to be sent to the email associated with the `access_token`.
pub fn request_operation_signing_token(pds: &str, access_token: &str) {
    super::api::plc::request_operation_signing_token(access_token, pds);
    println!(
        "PLC operation signature request submitted. Check associated email for confirmation code."
    );
}

/// Signs and submits a PLC operation.
///
/// TODO: this currently explicitly takes a `super::did::DidCreds` as the input data
/// for the operation, but it should really take an `Unknown`.
pub fn submit_signed_operation(
    pds: &str,
    access_token: &str,
    signing_token: &str,
    did_creds: super::api::did::RecommendedCredentials,
) {
    let res = plc::sign_operation(access_token, signing_token, did_creds, pds);
    plc::submit_plc_operation(access_token, res.operation, pds);
}
