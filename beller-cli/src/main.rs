mod cli;

use beller_lib::XRPC;
use clap::Parser;
use cli::{BellerCLI, Command, Credentials};

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
        Command::CreateSession { args } => {
            do_create_session(&args, &cli.pds);
        }
        Command::RequestPlcOperationSignature { auth_token } => {
            do_request_plc_operation_signature(&auth_token, &cli.pds);
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
