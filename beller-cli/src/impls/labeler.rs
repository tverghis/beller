use atrium_api::types::{DataModel, Unknown};
use ipld_core::ipld::Ipld;

use crate::impls::api::crypto::{retrieve_public_key, CurveAlgorithm};

/// Sets up the DID associated with the `access_token` to be a labeler service on the PDS.
pub fn setup(
    pds: &str,
    access_token: &str,
    signing_token: &str,
    labeler_url: &str,
    private_key: &str,
    key_alg: CurveAlgorithm,
) {
    let pub_key = retrieve_public_key(private_key, key_alg);

    let pub_key = DataModel::try_from(Ipld::String(pub_key))
        .expect("could not construct IPLD String for pub_key");

    let mut did_creds = super::api::did::get_recommended_credentials(access_token, pds);
    match &mut did_creds.verification_methods {
        None => {
            let map = Unknown::Object([("atproto_label".to_string(), pub_key)].into());
            did_creds.verification_methods = Some(map);
        }
        Some(Unknown::Object(m)) => {
            m.entry("atproto_label".to_string()).or_insert(pub_key);
        }
        _ => {
            panic!("Unexpected type for verification_methods");
        }
    };

    let lbl_svc_map = [
        ("type".to_string(), Ipld::String("AtprotoLabeler".into())),
        ("endpoint".to_string(), Ipld::String(labeler_url.into())),
    ];

    let lbl_svc_map = DataModel::try_from(Ipld::Map(lbl_svc_map.into()))
        .expect("unable to convert IPLD map to DataModel");

    match &mut did_creds.services {
        None => {
            did_creds.services = Some(Unknown::Object(
                [("atproto_labeler".to_string(), lbl_svc_map)].into(),
            ));
        }
        Some(Unknown::Object(m)) => {
            m.entry("atproto_labeler".to_string())
                .or_insert(lbl_svc_map);
        }
        _ => {
            panic!("Unexpected type for services");
        }
    };

    super::plc::submit_signed_operation(pds, access_token, signing_token, did_creds);

    eprintln!("Labeler setup complete.");
    super::repo::describe_session(access_token, pds);
}
