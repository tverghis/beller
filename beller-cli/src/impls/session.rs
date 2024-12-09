use beller_lib::XRPC;

use crate::cli::Credentials;

/// Creates a new session on the PDS.
pub fn create(args: &Credentials, pds: &str) {
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

/// Prints details about the current session.
pub fn get(access_token: &str, pds: &str) {
    match beller_lib::GetSession::new(access_token.to_string()).apply(pds) {
        Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
        Err(e) => {
            eprintln!("Error getting session: {e:?}");
            std::process::exit(1);
        }
    }
}
