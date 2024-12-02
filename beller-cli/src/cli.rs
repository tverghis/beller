#[derive(Debug, clap::Parser)]
pub struct BellerCLI {
    /// The command to execute.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Commands for interacting with the XRPC API.
    Api {
        #[command(subcommand)]
        commands: ApiCommands,

        /// URL of the personal data server.
        /// This may be specified on any command, if desired.
        #[arg(short = 's', long, default_value = "https://bsky.social")]
        pds: String,
    },

    /// Cryptography-related commands.
    #[command(subcommand)]
    Crypto(CryptoCommands),
}

#[derive(Debug, clap::Subcommand)]
pub enum ApiCommands {
    /// Create a session on the PDS.
    CreateSession {
        #[command(flatten)]
        args: Credentials,
    },

    /// Request a PLC operation signature.
    ///
    /// An email containing a confirmation code will be sent to the address
    /// associated with the credentials.
    RequestPlcOperationSignature {
        /// An access token obtained by invoking the `create_session`
        /// command.
        access_token: String,
    },

    /// Get credentials that should be in the DID document of the account.
    GetRecommendedDidCredentials {
        /// An access token obtained by invoking the `create_session`
        /// command.
        access_token: String,
    },
}

#[derive(Debug, clap::Subcommand)]
pub enum CryptoCommands {
    /// Generate a ECDSA private key.
    ///
    /// This command will always generate a key using the secp256k1 (k256)
    /// curve. The output is a base16-encoded string of the private key bytes.
    GeneratePrivateKey,

    /// Derive a public key from a private key.
    ///
    /// The public key will be encoded according to this specification:
    /// <https://atproto.com/specs/cryptography#public-key-encoding>.
    RetrievePublicKey {
        /// base16-encoded string of the private key bytes
        private_key: String,
    },
}

#[derive(Debug, Clone, clap::Args)]
pub struct Credentials {
    /// DID or handle of the labeler
    #[arg(short = 'u', long)]
    pub identifier: String,
    /// Password for the labeler
    #[arg(short = 'p', long)]
    pub password: String,
}
