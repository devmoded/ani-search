use crate::providers::shikimori::Orders;
use crate::{APP_NAME, types::Quality};
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
        #[command(subcommand)]
        command: KodikCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ShikimoriCommands {
    /// Поиск в базе Shikimori
    Search {
        /// Запрос.
        #[arg(short, long)]
        query: String,
        /// Лимит ответов.
        #[arg(short, long)]
        limit: u32,
        /// Порядок сортировки ответов.
        ///
        /// Допустимые значения [id, ranked, kind, popularity, name, aired_on, episodes, status, random]
        #[arg(short, long)]
        order: Orders,
    },
    /// Информация об указанном аниме
    Info {
        #[arg(short, long)]
        shikimori_id: u32,
    },
}

#[derive(Subcommand, Debug)]
pub enum KodikCommands {
    Find {
        #[arg(short, long)]
        shikimori_id: u32,
        /// Токен Kodik API.
        #[arg(long)]
        api_token: String,
    },
    Resolve {
        /// URL плеера Kodik (без указания протокола).
        #[arg(short, long)]
        player_url: String,
        /// Качество.
        ///
        /// Допустимые значения [low360p, sd480p, hd720p]
        #[arg(short, long)]
        quality: Quality,
    },
}
