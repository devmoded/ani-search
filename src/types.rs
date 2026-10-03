use serde::Serialize;
use std::fmt;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Serialize)]
pub struct ResolveResult {
    pub quality: Quality,
    pub m3u8: String,
}

#[derive(Debug, Clone, EnumString, Display, Serialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Quality {
    Hd720p,
    Sd480p,
    Low360p,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub releases: Vec<Release>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Release {
    pub title: String,
    pub title_ru: Option<String>,
    pub shikimori_id: Option<String>,
    pub translation: Option<Translation>,
    pub seasons: Option<Vec<Season>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Season {
    pub title: Option<String>,
    pub episodes: Vec<Episode>,
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.title {
            Some(title) => write!(f, "Сезон {}: {} эпизодов", title, self.episodes.len()),
            None => write!(f, "{} эпизодов", self.episodes.len()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Episode {
    pub num: u32,
    pub player_url: String,
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.num, self.player_url)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Translation {
    pub title: String,
    pub id: u32,
}

impl fmt::Display for Translation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.id)
    }
}
