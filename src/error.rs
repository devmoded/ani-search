use strum::{Display, EnumString};
use thiserror::Error;

#[derive(Debug, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Sources {
    Shikimori,
    Kodik,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("В базе {src} ничего не найдено по запросу {query}")]
    NotFound { src: Sources, query: String },
    #[error("Ошибка при парсинге {src}")]
    Parse { src: Sources },
}
