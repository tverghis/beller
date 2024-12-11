mod cli;
mod handlers;
mod impls;

use clap::Parser;
use cli::{BellerCLI, Commands};

fn main() {
    let cli = BellerCLI::parse();

    match cli.command {
        Commands::Api { commands, ref pds } => handlers::api_commands(commands, pds),
        Commands::Crypto(commands) => handlers::crypto_commands(commands),
        Commands::Labeler { commands, ref pds } => handlers::labeler_commands(commands, pds),
    };
}
