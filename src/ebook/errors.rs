use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum EbookError {
    #[error("file has no extension")]
    NoExtensionError,
    #[error("file extension {0} is not supported")]
    UnsupportedExtensionError(String),
}
