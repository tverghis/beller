mod cli;

use atrium_crypto::keypair::{Did, Export, Secp256k1Keypair};
use beller_lib::XRPC;
use clap::Parser;
use cli::{ApiCommands, BellerCLI, Commands, Credentials, CryptoCommands};
use multibase::Base;
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
    };
}

fn do_create_session(args: &Credentials, pds: &str) {
    match beller_lib::CreateSession::from(args).apply(pds) {
        Ok(res) => {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Err(e) => {
            println!("Error creating session: {e:?}");
            std::process::exit(1);
        }
    }
}

fn do_request_plc_operation_signature(auth_token: &str, pds: &str) {
    match beller_lib::RequestPlcOperationSignature::new(auth_token.to_string()).apply(pds) {
        Ok(()) => println!("PLC operation signature request submitted. Check associated email for confirmation code."),
        Err(e) => {
            println!("Error requesting PLC operation signature: {e:?}");
            std::process::exit(1);
        }
    }
}

fn do_get_recommended_did_credentials(access_token: &str, pds: &str) {
    match beller_lib::GetRecommendedDidCredentials::new(access_token.to_string()).apply(pds) {
        Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
        Err(e) => {
            println!("Error getting recommended DID credentials: {e:?}");
            std::process::exit(1);
        }
    }
}

fn do_generate_private_key() {
    let keypair = Secp256k1Keypair::create(&mut ThreadRng::default());
    let exported = keypair.export();
    let encoded = multibase::encode(multibase::Base::Base16Lower, &exported);
    println!("{}", encoded);
}

fn do_retrieve_public_key(private_key: &str) {
    match multibase::decode(private_key) {
        Ok((Base::Base16Lower, decoded)) => match Secp256k1Keypair::import(&decoded) {
            Ok(keypair) => println!("{}", keypair.did()),
            Err(e) => {
                println!("Error importing private key: {e:?}");
                std::process::exit(1);
            }
        },
        Ok((base, _)) => {
            println!("Unsupported base for private key: {base:?}");
            std::process::exit(1);
        }
        Err(e) => {
            println!("Error decoding private key: {e:?}");
            std::process::exit(1);
        }
    }
}
