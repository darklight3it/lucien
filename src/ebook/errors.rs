use epub::doc::DocError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EbookErrors {
    #[error("file has no extension")]
    NoExtensionError,
    #[error("file extension {0} is not supported")]
    UnsupportedExtensionError(String),
    #[error("not able to read epub document: {0}")]
    EpubDocError(#[from] DocError),
    #[error("did not founf any ISBN")]
    ISBNNotFoundError(),
}
