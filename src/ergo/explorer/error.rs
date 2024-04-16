use thiserror::Error;
use url::ParseError;
use std::io;
use reqwest::Error as ReqwestError;

#[derive(Debug, Error)]
pub enum ExplorerApiError {
    #[error("reqwest error: {0}")]
    RequestError(#[from] ReqwestError),

    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    
    #[error("invalid explorer url: {0}")]
    InvalidExplorerUrl(#[from] ParseError),
    
    #[error("IO error")]
    IOError(#[from] io::Error),

}