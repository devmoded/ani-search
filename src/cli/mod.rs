pub mod cli;
pub mod commands;

use anyhow::Result;

pub async fn run() -> Result<()> {
    commands::setup().await?;
    Ok(())
}
