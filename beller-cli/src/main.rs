mod cli;
mod impls;

use clap::Parser;
use cli::{ApiCommands, BellerCLI, Commands, Credentials, CryptoCommands, LabelerCommands};
use impls::{crypto, did, plc, repo, session};

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
            ApiCommands::DescribeRepo { did } => repo::describe_did(&did, &pds),
            ApiCommands::CreateSession { args } => {
                session::create(&args, &pds);
            }
            ApiCommands::GetSession { access_token } => {
                session::get(&access_token, &pds);
            }
            ApiCommands::RequestPlcOperationSignature { access_token } => {
                plc::request_operation_signing_token(&access_token, &pds);
            }
            ApiCommands::GetRecommendedDidCredentials { access_token } => {
                did::get_recommended_credentials(&access_token, &pds);
            }
        },
        Commands::Crypto(CryptoCommands::GeneratePrivateKey) => {
            crypto::print_private_key();
        }
        Commands::Crypto(CryptoCommands::RetrievePublicKey { private_key }) => {
            crypto::print_public_key(&private_key);
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
