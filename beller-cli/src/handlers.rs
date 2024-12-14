use crate::{
    cli::{ApiCommands, CryptoCommands, LabelerCommands},
    config::Configuration,
    impls::{crypto, did, labeler, plc, repo, session},
};

pub fn api_commands(commands: ApiCommands, config: &Configuration) {
    let pds = &config.pds.endpoint;

    match commands {
        ApiCommands::DescribeRepo { ref did } => repo::describe_did(pds, did),
        ApiCommands::CreateSession { ref args } => {
            session::create(pds, args);
        }
        ApiCommands::GetSession { ref access_token } => {
            session::get(pds, access_token);
        }
        ApiCommands::RequestPlcOperationSignature { ref access_token } => {
            plc::request_operation_signing_token(pds, access_token);
        }
        ApiCommands::GetRecommendedDidCredentials { ref access_token } => {
            did::get_recommended_credentials(pds, access_token);
        }
    }
}

pub fn crypto_commands(commands: CryptoCommands) {
    match commands {
        CryptoCommands::GeneratePrivateKey { alg } => crypto::print_private_key(alg),
        CryptoCommands::RetrievePublicKey { private_key, alg } => {
            crypto::print_public_key(&private_key, alg);
        }
    }
}

pub fn labeler_commands(commands: LabelerCommands, config: &Configuration) {
    match commands {
        LabelerCommands::Setup {
            access_token,
            signing_token,
            labeler_url,
            private_key,
            key_alg,
        } => labeler::setup(
            &config.pds.endpoint,
            &access_token,
            &signing_token,
            &labeler_url,
            &private_key,
            key_alg,
        ),
    }
}
