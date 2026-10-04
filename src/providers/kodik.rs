use crate::error::{Error, Sources};
use crate::types::{Episode, Quality, Release, ResolveResult, Response, Season, Translation};
use anyhow::{Context, Result};
use kodik_api::{
    Client as ApiClient,
    search::{SearchQuery, SearchResponse},
    types::EpisodeUnion,
};
use kodik_parser::reqwest::Client as ResolveClient;

#[derive(Debug)]
pub struct Kodik {
    api_client: Option<ApiClient>,
    resolve_client: ResolveClient,
}

impl Kodik {
    pub fn new() -> Self {
        Self {
            api_client: None,
            resolve_client: ResolveClient::new(),
        }
    }

    pub fn with_api_token(&mut self, api_token: impl Into<String>) -> &mut Self {
        self.api_client = Some(ApiClient::new(api_token));
        self
    }

    pub async fn find(&self, shikimori_id: &str) -> Result<Response> {
        let api_client = &self.api_client.clone().context(Error::ApiClientNotFound {
            src: Sources::Kodik,
        })?;

        let response = SearchQuery::new()
            .with_shikimori_id(shikimori_id)
            .with_episodes(true)
            .execute(api_client)
            .await
            .context(Error::NotFound {
                src: Sources::Kodik,
                query: format!("shikimori id: {}", shikimori_id),
            })?;

        anyhow::ensure!(
            !response.results.is_empty(),
            Error::NotFound {
                src: Sources::Kodik,
                query: format!("shikimori id: {}", shikimori_id)
            }
        );

        Ok(Response::from(&response))
    }

    pub async fn resolve(&self, player_url: &str, quality: &Quality) -> Result<ResolveResult> {
        let url = format!("https:{player_url}");
        let links = kodik_parser::parse(&self.resolve_client, &url)
            .await
            .context(Error::Resolve {
                url: url.clone(),
                src: Sources::Kodik,
            })?
            .links;

        let m3u8 = match quality {
            Quality::Hd720p => links
                .quality_720
                .first()
                .context(Error::Resolve {
                    url,
                    src: Sources::Kodik,
                })?
                .src
                .clone(),
            Quality::Sd480p => links
                .quality_480
                .first()
                .context(Error::Resolve {
                    url,
                    src: Sources::Kodik,
                })?
                .src
                .clone(),
            Quality::Low360p => links
                .quality_360
                .first()
                .context(Error::Resolve {
                    url,
                    src: Sources::Kodik,
                })?
                .src
                .clone(),
        };

        Ok(ResolveResult {
            quality: quality.clone(),
            m3u8,
        })
    }
}

impl From<&SearchResponse> for Response {
    fn from(r: &SearchResponse) -> Self {
        Response {
            releases: r.results.iter().map(|r| Release::from(r)).collect(),
        }
    }
}

impl From<&kodik_api::types::Release> for Release {
    fn from(r: &kodik_api::types::Release) -> Self {
        Release {
            title: r.title_orig.clone(),
            title_ru: Some(r.title.clone()),
            shikimori_id: match r.shikimori_id.clone() {
                Some(id) => Some(id.parse().expect("не удалось конвертировать shikimori id")),
                None => None,
            },
            translation: Some(Translation::from(&r.translation)),
            seasons: match &r.seasons {
                Some(seasons) => Some(seasons.iter().map(|(_, s)| Season::from(s)).collect()),
                // TODO: Попытаться сделать получше
                None => Some(vec![Season {
                    title: None,
                    episodes: vec![Episode {
                        num: 1,
                        player_url: r.link.clone(),
                    }],
                }]),
            },
        }
    }
}

impl From<&kodik_api::types::Translation> for Translation {
    fn from(t: &kodik_api::types::Translation) -> Self {
        Translation {
            title: t.title.clone(),
            id: t.id as u32,
        }
    }
}

impl From<&kodik_api::types::Season> for Season {
    fn from(s: &kodik_api::types::Season) -> Self {
        let mut episodes: Vec<Episode> = s
            .episodes
            .iter()
            .filter_map(|(key, ep)| {
                let num: u32 = key.parse().ok()?;
                let player_url = match ep {
                    EpisodeUnion::Link(url) => url.clone(),
                    EpisodeUnion::Episode(episode) => episode.link.clone(),
                };
                Some(Episode { num, player_url })
            })
            .collect();

        episodes.sort_by_key(|ep| ep.num);

        Season {
            title: s.title.clone(),
            episodes,
        }
    }
}
