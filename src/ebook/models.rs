use epub::doc::EpubDoc;
use mockall::automock;
use once_cell::sync::Lazy;
use regex::Regex;

use super::errors::EbookError;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// Supported Extensions
#[derive(Debug, PartialEq)]
pub enum SupportedExtensions {
    EPub,
}

impl TryFrom<&Path> for SupportedExtensions {
    type Error = EbookError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(EbookError::NoExtension)?;

        match extension.to_lowercase().as_str() {
            "epub" => Ok(SupportedExtensions::EPub),
            other => Err(EbookError::UnsupportedExtension(other.to_string())),
        }
    }
}

// Isbn
// we only validate the isbn broadly without implementing checksum

static ISBN10: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?i:(?:ISBN(?:-10)?:?\s*)?)\d{1,5}(?:[- ]?\d+){1,2}[- ]?[\dX]$").unwrap()
});

static ISBN13: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?i:(?:ISBN(?:-13)?:?\s*)?)97[89](?:[- ]?\d+){2,}[- ]?[\dX]$").unwrap()
});

#[derive(Debug, PartialEq)]
pub struct Isbn {
    id: String,
}

impl Isbn {
    pub fn new(raw: &String) -> Result<Self, EbookError> {
        if !Self::is_valid(raw) {
            return Err(EbookError::InvalidIsbn(raw.to_string()));
        }

        let id: String = raw
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == 'X') // allow X for ISBN-10
            .collect();

        Ok(Self { id })
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn is_valid(raw: &String) -> bool {
        ISBN10.is_match(raw) || ISBN13.is_match(raw)
    }
}

impl Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}

// EbookDoc
#[automock]
pub trait EbookDoc {
    fn mdata(&self, key: &str) -> Option<String>;
    fn metadata(&self) -> &HashMap<String, Vec<String>>;
}

impl EbookDoc for EpubDoc<BufReader<File>> {
    fn mdata(&self, key: &str) -> Option<String> {
        self.mdata(key)
    }

    fn metadata(&self) -> &HashMap<String, Vec<String>> {
        &self.metadata
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_isbn_constructor() {
        assert_eq!(
            "9781492056478",
            Isbn::new(&"978-1-492-05647-8".to_string())
                .unwrap()
                .get_id()
        );

        assert_eq!(
            "9781492056478",
            Isbn::new(&"978-1-492-05647-8".to_string())
                .unwrap()
                .get_id()
        );

        assert_eq!(
            "0198526636",
            Isbn::new(&"0-19-852663-6".to_string()).unwrap().get_id()
        );

        assert_eq!(
            "019852663X",
            Isbn::new(&"0-19-852663-X".to_string()).unwrap().get_id()
        );
    }

    #[test]
    fn test_supported_extensions_try_form() {
        assert!(matches!(
            SupportedExtensions::try_from(Path::new("/book.epub")),
            Ok(SupportedExtensions::EPub)
        ));

        assert!(matches!(
            SupportedExtensions::try_from(Path::new("/book")),
            Err(EbookError::NoExtension)
        ));

        assert!(matches!(
            SupportedExtensions::try_from(Path::new("/")),
            Err(EbookError::NoExtension)
        ));

        assert!(matches!(
            SupportedExtensions::try_from(Path::new("/book.jpeg")),
            Err(EbookError::UnsupportedExtension(ref ext)) if ext == "jpeg"
        ));
    }
}
