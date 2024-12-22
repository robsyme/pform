use reqwest::StatusCode;
use thiserror::Error;
use url::ParseError;
use crate::models::common::ValidationError;

#[derive(Error, Debug)]
pub enum SeqeraError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("Invalid URL: {0}")]
    Url(#[from] ParseError),
    
    #[error("Authentication failed")]
    Authentication,
    
    #[error("Access forbidden: you don't have permission to access this resource")]
    Forbidden,
    
    #[error("API error:\n  URL: {url}\n  Method: {method}\n  Status: {status}\n  Response: {message}")]
    Api {
        status: StatusCode,
        message: String,
        url: String,
        method: String,
    },
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}