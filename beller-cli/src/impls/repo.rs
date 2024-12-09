use beller_lib::XRPC;

/// Prints details about the repository for `did`.
pub fn describe(did: &str, pds: &str) {
    match beller_lib::DescribeRepo::new(did.to_string()).apply(pds) {
        Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
        Err(e) => {
            eprintln!("Error describing repository: {e:?}");
            std::process::exit(1);
        }
    }
}
