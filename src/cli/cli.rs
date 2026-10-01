use crate::APP_NAME;
use crate::providers::shikimori::Orders;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = APP_NAME, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Shikimori {
        #[arg(long)]
        api_url: Option<String>,
        #[command(subcommand)]
        command: ShikimoriCommands,
    },
    Kodik {
        #[arg(long)]
        api_token: String,
        #[command(subcommand)]
        command: KodikCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ShikimoriCommands {
    Search {
        #[arg(short, long)]
        query: String,
        #[arg(short, long)]
        limit: u32,
        /// Допустимые значения [id, ranked, kind, popularity, name, aired_on, episodes, status, random]
        #[arg(short, long)]
        order: Orders,
    },
    Info,
}

#[derive(Subcommand, Debug)]
pub enum KodikCommands {
    Find,
    Resolve,
}
