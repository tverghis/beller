mod cli;
mod crypto;
mod impls;

use atrium_api::types::{DataModel, Unknown};
use clap::Parser;
use cli::{ApiCommands, BellerCLI, Commands, Credentials, CryptoCommands, LabelerCommands};
use crypto::{generate_private_key, retrieve_public_key};
use ipld_core::ipld::Ipld;

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
            ApiCommands::DescribeRepo { did } => impls::repo::describe(&did, &pds),
            ApiCommands::CreateSession { args } => {
                impls::session::create(&args, &pds);
            }
            ApiCommands::GetSession { access_token } => {
                impls::session::get(&access_token, &pds);
            }
            ApiCommands::RequestPlcOperationSignature { access_token } => {
                impls::plc::request_operation_signing_token(&access_token, &pds);
            }
            ApiCommands::GetRecommendedDidCredentials { access_token } => {
                let creds = impls::did::get_recommended_credentials(&access_token, &pds);
                println!("{}", serde_json::to_string_pretty(&creds).unwrap());
            }
        },
        Commands::Crypto(CryptoCommands::GeneratePrivateKey) => {
            println!("{}", generate_private_key());
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

    let mut did_creds = impls::did::get_recommended_credentials(access_token, pds);
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

    impls::plc::submit_signed_operation(access_token, signing_token, did_creds, pds);

    eprintln!("Labeler setup complete.");
    impls::did::get_for_session(access_token, pds);
}
