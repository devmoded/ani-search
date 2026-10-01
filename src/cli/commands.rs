use crate::cli::cli::Cli;
use crate::cli::cli::Commands;
use crate::cli::cli::ShikimoriCommands;
use anyhow::Result;
use clap::Parser;

pub async fn setup() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Shikimori {
            api_url: shikimori_api_url,
            command,
        } => {
            println!("{}", shikimori_api_url.expect("not defined"));
            match command {
                ShikimoriCommands::Search => {}
                ShikimoriCommands::Info => {}
            }
        }
        Commands::Kodik { api_token, command } => {}
    }
    Ok(())
}
