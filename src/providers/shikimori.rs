use crate::error::{Error, Sources};
use crate::types::Release;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
struct Anime {
    id: u64,
    name: String,
    #[serde(default)]
    russian: String,
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
            shikimori_id: Some(a.id.to_string()),
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
}
