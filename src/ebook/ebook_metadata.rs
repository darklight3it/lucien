use std::path::Path;

use epub::doc::EpubDoc;

use crate::ebook::models::{EbookDoc, Isbn};

use super::{errors::EbookError, models::SupportedExtensions};

pub fn get_isbn(path: &Path) -> Result<Isbn, EbookError> {
    let extension = SupportedExtensions::try_from(path)?;
    match extension {
        SupportedExtensions::EPub => get_epub_isbn(path),
    }
}

fn get_epub_isbn(path: &Path) -> Result<Isbn, EbookError> {
    let doc = EpubDoc::new(path)?;
    get_isbn_from_doc(&doc)
}

fn get_isbn_from_doc<T: EbookDoc>(doc: &T) -> Result<Isbn, EbookError> {
    let raw = {
        let identifiers = doc.metadata().get("identifier");
        identifiers.and_then(|values| values.iter().find(|v| Isbn::is_valid(v)).cloned())
    };

    match raw {
        Some(r) => Ok(Isbn::new(&r)?),
        None => Err(EbookError::ISBNNotFound()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ebook::models::MockEbookDoc;

    #[test]
    fn test_get_isbn_from_doc_with_identifier() {
        let mut mock_doc = MockEbookDoc::new();
        mock_doc.expect_mdata().returning(|_| None);
        let mut metadata = std::collections::HashMap::new();
        metadata.insert(
            "identifier".to_string(),
            vec![
                "some-other-id".to_string(),
                "isbn:9781455509102".to_string(),
            ],
        );
        mock_doc.expect_metadata().times(1).return_const(metadata);

        let isbn = get_isbn_from_doc(&mock_doc).unwrap();
        assert_eq!(isbn, Isbn::new(&"isbn:9781455509102".to_string()).unwrap());
    }

    #[test]
    fn test_get_isbn_not_found() {
        let mut mock_doc = MockEbookDoc::new();
        mock_doc.expect_mdata().returning(|_| None);
        mock_doc
            .expect_metadata()
            .return_const(std::collections::HashMap::new());

        let err = get_isbn_from_doc(&mock_doc).unwrap_err();
        assert_eq!(err.to_string(), "could not find ISBN in ebook");
    }
}
