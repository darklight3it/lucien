use super::errors::EbookErrors;
use std::fmt::Display;
use std::path::Path;
#[derive(Debug, PartialEq)]
pub enum SupportedExtensions {
    EPub,
}

impl TryFrom<&Path> for SupportedExtensions {
    type Error = EbookErrors;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(EbookErrors::NoExtensionError)?;

        match extension.to_lowercase().as_str() {
            "epub" => Ok(SupportedExtensions::EPub),
            other => Err(EbookErrors::UnsupportedExtensionError(other.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ISBN {
    id: String,
}

impl ISBN {
    pub const PREFIX: &'static str = "urn:isbn:";
    pub fn new(raw: String) -> Self {
        let id: String = raw
            .strip_prefix(ISBN::PREFIX)
            .unwrap_or(&raw)
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == 'X') // allow X for ISBN-10
            .collect();

        return Self { id };
    }

    pub fn get_id(&self) -> &str {
        return &self.id;
    }

    fn get_long_id(&self) -> String {
        return format!("{}{}", ISBN::PREFIX, self.id);
    }
}

impl Display for ISBN {
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
            ISBN::new("978-1-492-05647-8".to_string()).id
        );

        assert_eq!(
            "9781492056478",
            ISBN::new(format!("{}{}", ISBN::PREFIX, "978 - 1 - 492 - 05647 - 8")).id
        );

        assert_eq!(
            "9781492056478",
            ISBN::new("978-1-492-05647-8".to_string()).id
        );

        assert_eq!("0198526636", ISBN::new("0-19-852663-6".to_string()).id);
        assert_eq!("019852663X", ISBN::new("0-19-852663-X".to_string()).id);

        assert_eq!(
            format!("{}{}", ISBN::PREFIX, "9781492056478"),
            ISBN::new("978-1-492-05647-8".to_string()).get_long_id()
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
            Err(EbookErrors::NoExtensionError)
        ));

        assert!(matches!(
            SupportedExtensions::try_from(Path::new("/")),
            Err(EbookErrors::NoExtensionError)
        ));

        assert!(matches!(
            SupportedExtensions::try_from(Path::new("/book.jpeg")),
            Err(EbookErrors::UnsupportedExtensionError(ref ext)) if ext == "jpeg"
        ));
    }
}
