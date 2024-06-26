use reqwest::StatusCode;
use thiserror::Error;

use crate::headers::Headers;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO {0}")]
    IO(#[from] std::io::Error),

    #[error("Timeout")]
    Timeout,

    #[error("Upload {0}")]
    Upload(String),

    #[error("Other {0}")]
    Other(String, Option<StatusCode>),

    #[error("reqwest {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("serde json {0}")]
    Json(#[from] serde_json::Error),

    #[error("TwitterError {0:?}, {1:?}, {1:?}")]
    Twitter(TwitterError, serde_json::Value, Box<Headers>), // https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
}

#[derive(Debug, Clone)]
pub struct TwitterError {
    pub status_code: StatusCode,
    pub status: u64,
    pub detail: String,
    pub title: String,
    pub r#type: String,
}

impl TwitterError {
    pub fn new(source: &serde_json::Value, status_code: StatusCode) -> Self {
        Self {
            status_code,
            status: source["status"].as_u64().unwrap_or_default(),
            detail: source["detail"].as_str().unwrap_or_default().to_owned(),
            title: source["title"].as_str().unwrap_or_default().to_owned(),
            r#type: source["type"].as_str().unwrap_or_default().to_owned(),
        }
    }
}
