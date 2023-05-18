use crate::twitter_api;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    TwitterApi(twitter_api::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(err) => write!(f, "{}", err),
            Self::SerdeJson(err) => write!(f, "{}", err),
            Self::TwitterApi(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error { }

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<twitter_api::Error> for Error {
    fn from(value: twitter_api::Error) -> Self {
        Self::TwitterApi(value)
    }
}