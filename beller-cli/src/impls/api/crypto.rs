use atrium_crypto::keypair::{Did, Export, Secp256k1Keypair};
use rand::rngs::ThreadRng;

const ENC_BASE: multibase::Base = multibase::Base::Base16Lower;

/// Generates a k256 ECDSA private key.
///
/// The output key is multibase `Base16Lower` encoded.
pub fn generate_private_key() -> String {
    let keypair = Secp256k1Keypair::create(&mut ThreadRng::default());
    let exported = keypair.export();
    multibase::encode(ENC_BASE, exported)
}

/// Given a k256 ECDSA key, encoded as multibase `Base16Lower`, derives the
/// corresponding public key.
pub fn retrieve_public_key(private_key: &str) -> String {
    match multibase::decode(private_key) {
        Ok((ENC_BASE, decoded)) => Secp256k1Keypair::import(&decoded)
            .expect("could not import private key")
            .did(),
        Ok((base, _)) => panic!("unsupported base {base:?} for private key"),
        Err(e) => panic!("failed to import private key: {e:?}"),
    }
}
