use std::path::Path;

use crate::ebook::{errors::EbookError, models::SupportedExtensions};

pub fn extract_medatadata(path: &Path) -> Result<(), EbookError> {
    if (path.is_file()) {
        let extension = SupportedExtensions::try_from(path)?;
        // supported extension is only epub at the moment, just going with the epub extraction method
    }

    todo!()
}
