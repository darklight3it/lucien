use super::errors::EbookError;
use std::fmt::Display;
use std::path::Path;
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

#[derive(Debug, PartialEq)]
pub struct Isbn {
    id: String,
}

impl Isbn {
    pub const PREFIX: &'static str = "urn:isbn:";
    pub fn new(raw: String) -> Self {
        let id: String = raw
            .strip_prefix(Isbn::PREFIX)
            .unwrap_or(&raw)
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == 'X') // allow X for ISBN-10
            .collect();

        Self { id }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_isbn_constructor() {
        assert_eq!(
            "9781492056478",
            Isbn::new("978-1-492-05647-8".to_string()).id
        );

        assert_eq!(
            "9781492056478",
            Isbn::new(format!("{}{}", Isbn::PREFIX, "978 - 1 - 492 - 05647 - 8")).id
        );

        assert_eq!(
            "9781492056478",
            Isbn::new("978-1-492-05647-8".to_string()).id
        );

        assert_eq!("0198526636", Isbn::new("0-19-852663-6".to_string()).id);
        assert_eq!("019852663X", Isbn::new("0-19-852663-X".to_string()).id);
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
