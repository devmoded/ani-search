use crate::cli::cli::Cli;
use crate::cli::cli::Commands;
use crate::cli::cli::{KodikCommands, ShikimoriCommands};
use crate::output::Output;
use crate::providers::kodik::Kodik;
use crate::providers::shikimori::Shikimori;
use anyhow::Result;
use clap::Parser;
use reqwest::Client;

pub async fn setup() -> Result<()> {
    let cli = Cli::parse();
    let output = Output::new();
    let reqwest_client = Client::builder()
        .user_agent(format!("{}-rust/{}", cli.client_name, cli.client_version))
        .build()?;

    match cli.command {
        Commands::Shikimori { api_url, command } => {
            let api_url = match api_url {
                Some(api) => api,
                None => "https://shikimori.io".to_string(),
            };
            let shikimori = Shikimori::new(&api_url, reqwest_client);
            match command {
                ShikimoriCommands::Search {
                    query,
                    limit,
                    order,
                } => {
                    let releases = shikimori.search(&query, limit, order).await?;

                    output.print(&releases)?;
                }
                ShikimoriCommands::Info { shikimori_id } => {
                    let anime = shikimori.info(&shikimori_id.to_string()).await?;

                    output.print(&anime)?;
                }
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

                    output.print(&response.releases)?;
                }
                KodikCommands::Resolve {
                    player_url,
                    quality,
                } => {
                    let res = kodik.resolve(&player_url, &quality).await?;

                    output.print(&res)?;
                }
            }
        }
    }
    Ok(())
}
