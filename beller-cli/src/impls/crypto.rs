pub use super::api::crypto::CurveAlgorithm;

/// Generates a new suitable private key and prints it to `stdout`.
pub fn print_private_key(alg: CurveAlgorithm) {
    let key = super::api::crypto::generate_private_key(alg);
    println!("{key}");
}

/// Given a private key, prints the associated public key to `stdout`.
pub fn print_public_key(private_key: &str, alg: CurveAlgorithm) {
    let key = super::api::crypto::retrieve_public_key(private_key, alg);
    println!("{key}");
}
