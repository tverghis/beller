use atrium_crypto::keypair::{Did, Secp256k1Keypair};
use multibase::Base;

pub fn retrieve_public_key(private_key: &str) -> anyhow::Result<String> {
    match multibase::decode(private_key) {
        Ok((Base::Base16Lower, decoded)) => Ok(Secp256k1Keypair::import(&decoded)?.did()),
        Ok((base, _)) => anyhow::bail!("unsupported base {:?} for private key", base),
        Err(e) => anyhow::bail!("failed to import private key: {:?}", e),
    }
}
