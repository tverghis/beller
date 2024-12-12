use atrium_crypto::{
    keypair::{Did, Export, P256Keypair, Secp256k1Keypair},
    Algorithm,
};
use rand::rngs::ThreadRng;

const ENC_BASE: multibase::Base = multibase::Base::Base16Lower;

/// Generates a private key.
///
/// The output key is multibase `Base16Lower` encoded.
pub fn generate_private_key(alg: Algorithm) -> String {
    let mut rng = ThreadRng::default();

    let exported = match alg {
        Algorithm::P256 => P256Keypair::create(&mut rng).export(),
        Algorithm::Secp256k1 => Secp256k1Keypair::create(&mut rng).export(),
    };

    multibase::encode(ENC_BASE, exported)
}

/// Given a key, encoded as multibase `Base16Lower`, derives the
/// corresponding public key.
pub fn retrieve_public_key(private_key: &str, alg: Algorithm) -> String {
    match multibase::decode(private_key) {
        Ok((ENC_BASE, decoded)) => match alg {
            Algorithm::P256 => P256Keypair::import(&decoded)
                .expect("could not import p256 private key")
                .did(),
            Algorithm::Secp256k1 => Secp256k1Keypair::import(&decoded)
                .expect("could not import k256 private key")
                .did(),
        },
        Ok((base, _)) => panic!("unsupported base {base:?} for private key"),
        Err(e) => panic!("failed to import private key: {e:?}"),
    }
}
