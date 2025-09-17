use std::path::Path;

use crate::ebook::errors::EbookError;
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
            .ok_or(EbookError::NoExtensionError)?;

        match extension.to_lowercase().as_str() {
            "epub" => Ok(SupportedExtensions::EPub),
            other => Err(EbookError::UnsupportedExtensionError(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_extensions_try_form() {
        assert_eq!(
            Ok(SupportedExtensions::EPub),
            SupportedExtensions::try_from(Path::new("/book.epub"))
        );

        assert_eq!(
            Err(EbookError::NoExtensionError),
            SupportedExtensions::try_from(Path::new("/book"))
        );

        assert_eq!(
            Err(EbookError::NoExtensionError),
            SupportedExtensions::try_from(Path::new("/"))
        );

        assert_eq!(
            Err(EbookError::UnsupportedExtensionError("jpeg".to_string())),
            SupportedExtensions::try_from(Path::new("/book.jpeg"))
        );
    }
}
