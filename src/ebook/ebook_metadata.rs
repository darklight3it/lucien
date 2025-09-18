use std::path::Path;

use epub::doc::EpubDoc;

use crate::ebook::models::Isbn;

use super::{errors::EbookError, models::SupportedExtensions};

pub fn get_isbn(path: &Path) -> Result<Isbn, EbookError> {
    let extension = SupportedExtensions::try_from(path)?;
    match extension {
        SupportedExtensions::EPub => get_epub_isbn(path),
    }
}

fn get_epub_isbn(path: &Path) -> Result<Isbn, EbookError> {
    let doc = EpubDoc::new(path)?;

    let raw = doc.mdata("pub-id").or_else(|| doc.mdata("book-id"));

    match raw {
        Some(r) => Ok(Isbn::new(r)),
        None => Err(EbookError::ISBNNotFound()),
    }
}
