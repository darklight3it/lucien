use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum EbookErrors {
    #[error("file has no extension")]
    NoExtensionError,
    #[error("file extension {0} is not supported")]
    UnsupportedExtensionError(String),
}
