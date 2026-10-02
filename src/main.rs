mod cli;
mod error;
mod output;
mod providers;
mod types;

use anyhow::Result;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await?;
    Ok(())
}
