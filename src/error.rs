use thiserror::Error;

#[derive(Error, Debug)]
pub enum AzureError {
    #[error("Failed to get access token")]
    TokenError,
    #[error("Failed to parse subscriptions")]
    ParseError,
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid response format")]
    InvalidResponse,
}