use atrium_api::{
    com::atproto::identity::{sign_plc_operation, submit_plc_operation},
    types::Unknown,
};
use beller_lib::XRPC;

/// Requests an operation signing token to be sent to the email associated with the `access_token`.
pub fn request_operation_signing_token(access_token: &str, pds: &str) {
    beller_lib::RequestPlcOperationSignature::new(access_token.to_string())
        .apply(pds)
        .expect("could not request PLC operation signing token")
}

/// Signs a PLC operation.
///
/// TODO: this currently explicitly takes a `super::did::DidCreds` as the input data
/// for the operation, but it should really take an `Unkown`.
pub fn sign_operation(
    access_token: &str,
    signing_token: &str,
    did_creds: super::did::DidCreds,
    pds: &str,
) -> <beller_lib::SignPlcOperation as XRPC>::Return {
    let input = sign_plc_operation::InputData {
        also_known_as: did_creds.also_known_as,
        rotation_keys: did_creds.rotation_keys,
        services: did_creds.services,
        verification_methods: did_creds.verification_methods,
        token: Some(signing_token.to_string()),
    };

    beller_lib::SignPlcOperation::new(access_token.to_string(), input)
        .apply(pds)
        .expect("could not sign PLC operation")
}

/// Submits a signed PLC `operation`.
pub fn submit_plc_operation(access_token: &str, operation: Unknown, pds: &str) {
    let submit_op_input = submit_plc_operation::InputData { operation };
    beller_lib::SubmitPlcOperation::new(access_token.to_string(), submit_op_input)
        .apply(pds)
        .expect("could not submit PLC operation")
}
