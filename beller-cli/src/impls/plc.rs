use atrium_api::{
    com::atproto::identity::{sign_plc_operation, submit_plc_operation},
    types::Unknown,
};
use beller_lib::XRPC;

/// Requests an operation signing token to be sent to the email associated with the `access_token`.
pub fn request_operation_signing_token(access_token: &str, pds: &str) {
    match beller_lib::RequestPlcOperationSignature::new(access_token.to_string()).apply(pds) {
        Ok(()) => println!("PLC operation signature request submitted. Check associated email for confirmation code."),
        Err(e) => {
            eprintln!("Error requesting PLC operation signature: {e:?}");
            std::process::exit(1);
        }
    }
}

/// Signs and submits a PLC operation.
///
/// TODO: this currently explicitly takes a `super::did::DidCreds` as the input data
/// for the operation, but it should really take an `Unkown`.
pub fn submit_signed_operation(
    access_token: &str,
    signing_token: &str,
    did_creds: super::did::DidCreds,
    pds: &str,
) {
    let res = sign_operation(access_token, signing_token, did_creds, pds);
    submit_plc_operation(access_token, res.operation, pds);
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

    match beller_lib::SignPlcOperation::new(access_token.to_string(), input).apply(pds) {
        Ok(res) => res,
        Err(e) => {
            match e.into_response() {
                Some(resp) => {
                    eprintln!(
                        "Failed to perform `{}`: {}",
                        beller_lib::SignPlcOperation::NSID,
                        // It doesn't look like atrium-api's `Error` enum is fully implemented, so we
                        // directly access the response body here.
                        resp.into_json::<serde_json::Value>()
                            .unwrap()
                            .get("message")
                            .unwrap()
                    );
                }
                None => {
                    eprintln!("Failed to perform `{}`", beller_lib::SignPlcOperation::NSID);
                }
            }
            std::process::exit(1);
        }
    }
}

/// Submits a signed PLC `operation`.
pub fn submit_plc_operation(access_token: &str, operation: Unknown, pds: &str) {
    let submit_op_input = submit_plc_operation::InputData { operation };
    if let Err(e) =
        beller_lib::SubmitPlcOperation::new(access_token.to_string(), submit_op_input).apply(pds)
    {
        eprintln!(
            "Failed to perform `{}`: {e}",
            beller_lib::SubmitPlcOperation::NSID
        );
        std::process::exit(1);
    }
}
