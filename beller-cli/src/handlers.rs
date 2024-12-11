use crate::{
    cli::{ApiCommands, CryptoCommands, LabelerCommands},
    impls::{crypto, did, labeler, plc, repo, session},
};

pub fn api_commands(commands: ApiCommands, pds: &str) {
    match commands {
        ApiCommands::DescribeRepo { did } => repo::describe_did(&did, pds),
        ApiCommands::CreateSession { args } => {
            session::create(&args, pds);
        }
        ApiCommands::GetSession { access_token } => {
            session::get(&access_token, pds);
        }
        ApiCommands::RequestPlcOperationSignature { access_token } => {
            plc::request_operation_signing_token(&access_token, pds);
        }
        ApiCommands::GetRecommendedDidCredentials { access_token } => {
            did::get_recommended_credentials(&access_token, pds);
        }
    }
}

pub fn crypto_commands(commands: CryptoCommands) {
    match commands {
        CryptoCommands::GeneratePrivateKey => crypto::print_private_key(),
        CryptoCommands::RetrievePublicKey { private_key } => crypto::print_public_key(&private_key),
    }
}

pub fn labeler_commands(commands: LabelerCommands, pds: &str) {
    match commands {
        LabelerCommands::Setup {
            access_token,
            signing_token,
            labeler_url,
            private_key,
        } => labeler::setup(
            &access_token,
            &signing_token,
            &labeler_url,
            &private_key,
            pds,
        ),
    }
}
