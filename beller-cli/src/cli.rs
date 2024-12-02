#[derive(Debug, clap::Parser)]
pub struct BellerCLI {
    /// The command to execute.
    #[command(subcommand)]
    pub command: Command,

    /// URL of the personal data server.
    /// This may be specified on any command, if desired.
    #[arg(short = 's', long, default_value = "https://bsky.social")]
    pub pds: String,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
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

    /// Generate a ECDSA private key.
    ///
    /// This command will always generate a key using the secp256k1 (k256)
    /// curve. The output is a base16-encoded string of the private key bytes.
    GeneratePrivateKey,
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
