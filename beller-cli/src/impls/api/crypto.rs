use atrium_crypto::keypair::{Did, Export, P256Keypair, Secp256k1Keypair};
use clap::ValueEnum;
use rand::rngs::ThreadRng;

const ENC_BASE: multibase::Base = multibase::Base::Base16Lower;

/// Supported ECDSA cryptography algorithms
///
/// See the [Cryptography](https://atproto.com/specs/cryptography) section of
/// the protocol for more details.
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum CurveAlgorithm {
    /// NIST P-256 aka secp256r1 aka prime256v1
    P256,
    /// NIST K-256 aka secp256k1
    K256,
}

impl Default for CurveAlgorithm {
    fn default() -> Self {
        Self::K256
    }
}

/// Generates a private key.
///
/// The output key is multibase `Base16Lower` encoded.
pub fn generate_private_key(alg: CurveAlgorithm) -> String {
    let mut rng = ThreadRng::default();

    let exported = match alg {
        CurveAlgorithm::P256 => P256Keypair::create(&mut rng).export(),
        CurveAlgorithm::K256 => Secp256k1Keypair::create(&mut rng).export(),
    };

    multibase::encode(ENC_BASE, exported)
}

/// Given a key, encoded as multibase `Base16Lower`, derives the
/// corresponding public key.
pub fn retrieve_public_key(private_key: &str, alg: CurveAlgorithm) -> String {
    match multibase::decode(private_key) {
        Ok((ENC_BASE, decoded)) => match alg {
            CurveAlgorithm::P256 => P256Keypair::import(&decoded)
                .expect("could not import p256 private key")
                .did(),
            CurveAlgorithm::K256 => Secp256k1Keypair::import(&decoded)
                .expect("could not import k256 private key")
                .did(),
        },
        Ok((base, _)) => panic!("unsupported base {base:?} for private key"),
        Err(e) => panic!("failed to import private key: {e:?}"),
    }
}
