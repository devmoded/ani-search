use crate::cli::cli::Cli;
use crate::cli::cli::Commands;
use crate::cli::cli::ShikimoriCommands;
use crate::providers::shikimori::Shikimori;
use crate::{APP_NAME, APP_VERSION};
use anyhow::Result;
use clap::Parser;
use reqwest::Client;

pub async fn setup() -> Result<()> {
    let cli = Cli::parse();
    let reqwest_client = Client::builder()
        .user_agent(format!("{APP_NAME}-rust/{APP_VERSION}"))
        .build()?;

    match cli.command {
        Commands::Shikimori {
            api_url: shikimori_api_url,
            command,
        } => {
            let api_url = match shikimori_api_url {
                Some(api) => api,
                None => "https://shikimori.io/api/animes".to_string(),
            };
            match command {
                ShikimoriCommands::Search {
                    query,
                    limit,
                    order,
                } => {
                    let releases = Shikimori::new(&api_url, reqwest_client)
                        .search(&query, limit, order)
                        .await?;
                    println!("{:#?}", releases)
                }
                ShikimoriCommands::Info => {}
            }
        }
        Commands::Kodik { api_token, command } => {}
    }
    Ok(())
}
