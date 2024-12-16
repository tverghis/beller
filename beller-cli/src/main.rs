mod cli;
mod config;
mod handlers;
mod impls;

use clap::Parser;
use cli::{BellerCLI, Commands};
use config::{ConfigOption, Configuration};

fn main() {
    let cli = BellerCLI::parse();

    let mut config = Configuration::from_file();

    match cli.command {
        Commands::Api { commands, pds } => {
            handlers::api_commands(&commands, config.apply(ConfigOption::PdsEndpoint(pds)));
        }
        Commands::Crypto(commands) => handlers::crypto_commands(&commands),
        Commands::Labeler { commands, pds } => {
            handlers::labeler_commands(&commands, config.apply(ConfigOption::PdsEndpoint(pds)));
        }
    };
}
