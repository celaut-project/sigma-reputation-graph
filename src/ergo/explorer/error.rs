use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum ExplorerApiError {
    #[error("reqwest error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("invalid explorer url: {0}")]
    InvalidExplorerUrl(#[from] ParseError),
}