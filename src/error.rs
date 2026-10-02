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
    #[error("У {src} не найден API клиент")]
    ApiClientNotFound { src: Sources },
    #[error("Ошибка резолвинга {url} в {src}")]
    Resolve { src: Sources, url: String },
    #[error("Ошибка при парсинге {src}")]
    Parse { src: Sources },
}
