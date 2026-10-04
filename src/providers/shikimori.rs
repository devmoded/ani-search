use crate::error::{Error, Sources};
use crate::types::Release;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Orders {
    Id,
    Ranked,
    Kind,
    Popularity,
    Name,
    AiredOn,
    Episodes,
    Status,
    Random,
}

#[derive(Debug, Display, EnumString, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Kind {
    Tv,
    Movie,
    Ova,
    Ona,
    Special,
    TvSpecial,
    Music,
    Pv,
    Cm,
    #[serde(rename = "tv_13")]
    Tv13,
    #[serde(rename = "tv_24")]
    Tv24,
    #[serde(rename = "tv_48")]
    Tv48,
}

#[derive(Debug, Display, EnumString, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    Anons,
    Ongoing,
    Released,
}

#[derive(Debug, Display, EnumString, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Rating {
    None,
    G,
    Pg,
    #[serde(rename = "pg_13")]
    Pg13,
    R,
    RPlus,
    Rx,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Image {
    original: String,
    preview: String,
    x96: String,
    x48: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Anime {
    id: u64,
    name: String,
    #[serde(default)]
    russian: String,
    image: Image,
    url: String,
    kind: Kind,
    score: String,
    status: Status,
    episodes: u32,
    episodes_aired: u32,
    aired_on: Option<String>,
    released_on: Option<String>,
    rating: Option<Rating>,
    duration: Option<u32>,
    description: Option<String>,
    anons: Option<bool>,
    ongoing: Option<bool>,
    myanimelist_id: Option<u32>,
}

impl From<Anime> for Release {
    fn from(a: Anime) -> Self {
        return Release {
            title: a.name.clone(),
            title_ru: if !a.russian.is_empty() {
                Some(a.russian.clone())
            } else {
                None
            },
            shikimori_id: Some(a.id),
            translation: None,
            seasons: None,
        };
    }
}

pub struct Shikimori {
    api_url: String,
    reqwest_client: Client,
}

impl Shikimori {
    pub fn new(api_url: &str, reqwest_client: Client) -> Self {
        Self {
            api_url: api_url.to_string(),
            reqwest_client,
        }
    }

    pub async fn search(&self, query: &str, limit: u32, order: Orders) -> Result<Vec<Release>> {
        let response: Vec<Release> = self
            .reqwest_client
            .get(&format!("{}{}", &self.api_url, "/api/animes"))
            .query(&[
                ("search", query),
                ("limit", &limit.to_string()),
                ("order", &order.to_string()),
            ])
            .send()
            .await
            .context(Error::NotFound {
                src: Sources::Shikimori,
                query: query.to_string(),
            })?
            .json::<Vec<Anime>>()
            .await
            .context(Error::Parse {
                src: Sources::Shikimori,
            })?
            .into_iter()
            .map(Into::into)
            .collect();

        anyhow::ensure!(
            !(response.len() == 0),
            Error::NotFound {
                src: Sources::Shikimori,
                query: query.to_string(),
            }
        );

        Ok(response)
    }

    pub async fn info(&self, shikimori_id: &str) -> Result<Anime> {
        let response: Anime = self
            .reqwest_client
            .get(&format!(
                "{}{}{}",
                &self.api_url, "/api/animes/", shikimori_id
            ))
            .send()
            .await
            .context(Error::NotFound {
                src: Sources::Shikimori,
                query: format!("shikimori id: {}", shikimori_id),
            })?
            .json()
            .await
            .context(Error::Parse {
                src: Sources::Shikimori,
            })?;

        Ok(response)
    }
}
