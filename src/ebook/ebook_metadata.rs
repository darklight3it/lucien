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
    let raw = doc
        .mdata("pub-id")
        .or_else(|| doc.mdata("book-id"))
        .or_else(|| try_get_isbn_from_identifiers(doc));

    match raw {
        Some(r) => Ok(Isbn::new(r)),
        None => Err(EbookError::ISBNNotFound()),
    }
}

fn try_get_isbn_from_identifiers<T: EbookDoc>(doc: &T) -> Option<String> {
    let identifiers = doc.metadata().get("identifier");
    identifiers.and_then(|values| {
        values
            .iter()
            .find(|v| v.to_lowercase().starts_with("isbn:"))
            .cloned()
    })
}

#[cfg(test)]
mod tests {
    use crate::ebook::models::MockEbookDoc;

    use super::*;
    use mockall::*;

    const ISBN: &str = "9781455509102";

    #[test]
    fn test_get_isbn_from_doc_with_pub_id() {
        let mut mock_doc = MockEbookDoc::new();
        mock_doc
            .expect_mdata()
            .with(predicate::eq("pub-id"))
            .times(1)
            .returning(|_| Some(ISBN.to_string()));

        let isbn = get_isbn_from_doc(&mock_doc).unwrap();
        assert_eq!(isbn, Isbn::new(ISBN.to_string()));
    }

    #[test]
    fn test_get_isbn_from_doc_with_book_id() {
        let mut mock_doc = MockEbookDoc::new();
        mock_doc
            .expect_mdata()
            .with(predicate::eq("pub-id"))
            .times(1)
            .returning(|_| None);
        mock_doc
            .expect_mdata()
            .with(predicate::eq("book-id"))
            .times(1)
            .returning(|_| Some(ISBN.to_string()));

        let isbn = get_isbn_from_doc(&mock_doc).unwrap();
        assert_eq!(isbn, Isbn::new(ISBN.to_string()));
    }

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
        assert_eq!(isbn, Isbn::new("isbn:9781455509102".to_string()));
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
