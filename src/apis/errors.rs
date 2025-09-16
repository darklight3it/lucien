use reqwest::StatusCode;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] SerdeError),

    #[error("API returned an error: {status} - {message}")]
    Api { status: StatusCode, message: String },
}
