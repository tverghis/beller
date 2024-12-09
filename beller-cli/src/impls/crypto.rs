/// Generates a new suitable private key and prints it to `stdout`.
pub fn print_private_key() {
    let key = super::api::crypto::generate_private_key();
    println!("{key}");
}

/// Given a private key, prints the associated public key to `stdout`.
pub fn print_public_key(private_key: &str) {
    let key = super::api::crypto::retrieve_public_key(private_key);
    println!("{key}");
}
