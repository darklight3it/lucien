use std::path::Path;

use epub::doc::EpubDoc;

use crate::ebook::models::ISBN;

use super::{errors::EbookErrors, models::SupportedExtensions};

pub fn get_isbn(path: &Path) -> Result<ISBN, EbookErrors> {
    let extension = SupportedExtensions::try_from(path)?;
    return match extension {
        SupportedExtensions::EPub => get_epub_isbn(path),
    };
}

fn get_epub_isbn(path: &Path) -> Result<ISBN, EbookErrors> {
    let doc = EpubDoc::new(path)?;

    let raw = doc.mdata("pub-id").or_else(|| doc.mdata("book-id"));

    return match raw {
        Some(r) => Ok(ISBN::new(r)),
        None => Err(EbookErrors::ISBNNotFoundError()),
    };
}
