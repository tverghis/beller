mod cli;
mod crypto;

use atrium_api::types::{DataModel, Unknown};
use atrium_crypto::keypair::{Export, Secp256k1Keypair};
use beller_lib::XRPC;
use clap::Parser;
use cli::{ApiCommands, BellerCLI, Commands, Credentials, CryptoCommands, LabelerCommands};
use crypto::retrieve_public_key;
use ipld_core::ipld::Ipld;
use rand::rngs::ThreadRng;

impl From<&Credentials> for beller_lib::CreateSession {
    fn from(args: &Credentials) -> Self {
        Self {
            identifier: args.identifier.clone(),
            password: args.password.clone(),
        }
    }
}

fn main() {
    let cli = BellerCLI::parse();

    match cli.command {
        Commands::Api { commands, pds } => match commands {
            ApiCommands::CreateSession { args } => {
                do_create_session(&args, &pds);
            }
            ApiCommands::RequestPlcOperationSignature { access_token } => {
                do_request_plc_operation_signature(&access_token, &pds);
            }
            ApiCommands::GetRecommendedDidCredentials { access_token } => {
                do_get_recommended_did_credentials(&access_token, &pds);
            }
        },
        Commands::Crypto(CryptoCommands::GeneratePrivateKey) => {
            do_generate_private_key();
        }
        Commands::Crypto(CryptoCommands::RetrievePublicKey { private_key }) => {
            do_retrieve_public_key(&private_key);
        }
        Commands::Labeler { commands, pds } => match commands {
            LabelerCommands::Setup {
                access_token,
                signing_token,
                labeler_url,
                private_key,
            } => do_setup_labeler(
                &access_token,
                &signing_token,
                &labeler_url,
                &private_key,
                &pds,
            ),
        },
    };
}

fn do_create_session(args: &Credentials, pds: &str) {
    match beller_lib::CreateSession::from(args).apply(pds) {
        Ok(res) => {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Err(e) => {
            eprintln!("Error creating session: {e:?}");
            std::process::exit(1);
        }
    }
}

fn do_request_plc_operation_signature(auth_token: &str, pds: &str) {
    match beller_lib::RequestPlcOperationSignature::new(auth_token.to_string()).apply(pds) {
        Ok(()) => println!("PLC operation signature request submitted. Check associated email for confirmation code."),
        Err(e) => {
            eprintln!("Error requesting PLC operation signature: {e:?}");
            std::process::exit(1);
        }
    }
}

fn do_get_recommended_did_credentials(access_token: &str, pds: &str) {
    match beller_lib::GetRecommendedDidCredentials::new(access_token.to_string()).apply(pds) {
        Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
        Err(e) => {
            eprintln!("Error getting recommended DID credentials: {e:?}");
            std::process::exit(1);
        }
    }
}

// TODO: move internals to crypto module
fn do_generate_private_key() {
    let keypair = Secp256k1Keypair::create(&mut ThreadRng::default());
    let exported = keypair.export();
    let encoded = multibase::encode(multibase::Base::Base16Lower, exported);
    println!("{encoded}");
}

fn do_retrieve_public_key(private_key: &str) {
    match retrieve_public_key(private_key) {
        Ok(key) => println!("{key}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn do_setup_labeler(
    access_token: &str,
    signing_token: &str,
    labeler_url: &str,
    private_key: &str,
    pds: &str,
) {
    let Ok(pub_key) = retrieve_public_key(private_key) else {
        eprintln!("Failed to retrieve public key from the provided private key.");
        std::process::exit(1);
    };

    let pub_key = DataModel::try_from(Ipld::String(pub_key))
        .expect("could not construct IPLD String for pub_key");

    let Ok(mut did_creds) =
        beller_lib::GetRecommendedDidCredentials::new(access_token.into()).apply(pds)
    else {
        eprintln!(
            "Failed to perform `{}`",
            beller_lib::GetRecommendedDidCredentials::NSID
        );
        std::process::exit(1);
    };

    match &mut did_creds.verification_methods {
        None => {
            let map = Unknown::Object([("atproto_label".to_string(), pub_key)].into());
            did_creds.verification_methods = Some(map);
        }
        Some(Unknown::Object(m)) => {
            m.entry("atproto_label".to_string()).or_insert(pub_key);
        }
        _ => {
            eprintln!("Unexpected type for verification_methods");
            std::process::exit(1);
        }
    }

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
            eprintln!("Unexpected type for services");
            std::process::exit(1);
        }
    }

    dbg!(did_creds);
}
