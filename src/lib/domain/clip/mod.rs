pub mod field;

use serde:: {Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Invalid title: {0}")]
    InvalidTitle(String),
    #[error("Empty Content")]
    EmptyContent,
    #[error("invalid date: {0}")]
    InvaidDate(String),
    #[error("Date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("id parse error: {0}")]
    Id(#[from]uuid::Error),
    #[error("Hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

#[derive(Debug, CLone,Deserialize,Serialize)]

pub struct Clip {
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}

