use std::fmt;

#[derive(Debug, Clone)]
pub struct Response {
    pub releases: Vec<Release>,
}

#[derive(Debug, Clone)]
pub struct Release {
    pub title: String,
    pub title_ru: Option<String>,
    pub shikimori_id: Option<String>,
    pub translation: Option<Translation>,
    pub seasons: Option<Vec<Season>>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Episode {
    pub num: u32,
    pub url: String,
}

impl Episode {
    pub fn new(num: u32, url: &str) -> Self {
        Self {
            num,
            url: url.to_string(),
        }
    }
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.num, self.url)
    }
}

#[derive(Debug, Clone)]
pub struct Translation {
    pub title: String,
    pub id: u32,
}

impl fmt::Display for Translation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.id)
    }
}
