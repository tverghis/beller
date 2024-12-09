mod cli;
mod crypto;
mod impls;

use clap::Parser;
use cli::{ApiCommands, BellerCLI, Commands, Credentials, CryptoCommands, LabelerCommands};
use crypto::{generate_private_key, retrieve_public_key};

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
            ApiCommands::DescribeRepo { did } => impls::repo::describe_did(&did, &pds),
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
                impls::did::get_recommended_credentials(&access_token, &pds);
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
            } => impls::labeler::setup(
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
