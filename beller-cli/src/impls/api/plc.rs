use atrium_api::{
    com::atproto::identity::{sign_plc_operation, submit_plc_operation},
    types::Unknown,
};
use beller_lib::{identity, XRPC};

use crate::impls::defs::PdsUrl;

/// Requests an operation signing token to be sent to the email associated with the `access_token`.
pub fn request_operation_signing_token(pds: &PdsUrl, access_token: &str) {
    identity::RequestPlcOperationSignature::new(access_token.to_string())
        .apply(pds.into())
        .expect("could not request PLC operation signing token");
}

/// Signs a PLC operation.
///
/// TODO: this currently explicitly takes a `super::did::DidCreds` as the input data
/// for the operation, but it should really take an `Unknown`.
pub fn sign_operation(
    pds: &PdsUrl,
    access_token: &str,
    signing_token: &str,
    did_creds: super::did::RecommendedCredentials,
) -> <beller_lib::identity::SignPlcOperation as XRPC>::Return {
    let input = sign_plc_operation::InputData {
        also_known_as: did_creds.also_known_as,
        rotation_keys: did_creds.rotation_keys,
        services: did_creds.services,
        verification_methods: did_creds.verification_methods,
        token: Some(signing_token.to_string()),
    };

    identity::SignPlcOperation::new(access_token.to_string(), input)
        .apply(pds.into())
        .expect("could not sign PLC operation")
}

/// Submits a signed PLC `operation`.
pub fn submit_plc_operation(pds: &PdsUrl, access_token: &str, operation: Unknown) {
    let submit_op_input = submit_plc_operation::InputData { operation };
    identity::SubmitPlcOperation::new(access_token.to_string(), submit_op_input)
        .apply(pds.into())
        .expect("could not submit PLC operation");
}
