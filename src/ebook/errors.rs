use epub::doc::DocError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EbookError {
    #[error("file has no extension")]
    NoExtension,
    #[error("file extension {0} is not supported")]
    UnsupportedExtension(String),
    #[error("not able to read epub document: {0}")]
    EpubDoc(#[from] DocError),
    #[error("did not found any ISBN in the file")]
    ISBNNotFound(),
}
