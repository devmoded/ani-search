use crate::cli::cli::Cli;
use crate::cli::cli::Commands;
use crate::cli::cli::{KodikCommands, ShikimoriCommands};
use crate::providers::kodik::Kodik;
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
        Commands::Shikimori { api_url, command } => {
            let api_url = match api_url {
                Some(api) => api,
                None => "https://shikimori.io".to_string(),
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
        Commands::Kodik { command } => {
            let mut kodik = Kodik::new();

            match command {
                KodikCommands::Find {
                    shikimori_id,
                    api_token,
                } => {
                    let response = kodik
                        .with_api_token(api_token)
                        .find(&shikimori_id.to_string())
                        .await?;

                    let releases = response.releases;

                    println!("{:#?}", releases)
                }
                KodikCommands::Resolve {
                    player_url,
                    quality,
                } => {
                    let m3u8 = kodik.resolve_link(&player_url, &quality).await?;

                    println!("{:#?}", m3u8)
                }
            }
        }
    }
    Ok(())
}
