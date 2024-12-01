use clap::Parser;

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Request a PLC operation signature.
    /// 
    /// An email containing a confirmation code will be sent to the address
    /// associated with the credentials.
    RequestPlcOperationSignature {
        /// The personal data server that hosts the labeler's repo.
        #[arg(short = 's', long, default_value = "https://bsky.social")]
        pds: Option<String>,
        /// DID or handle of the labeler
        #[arg(short = 'u', long)]
        identifier: String,
        /// Password for the labeler
        #[arg(short = 'p', long)]
        password: String,
    },
}

#[derive(Debug, clap::Parser)]
struct BellerCLI {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let _ = BellerCLI::parse();
}
