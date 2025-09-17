use std::path::Path;

use super::{errors::EbookErrors, models::SupportedExtensions};

pub fn extract_medatadata(path: &Path) -> Result<(), EbookErrors> {
    if (path.is_file()) {
        let extension = SupportedExtensions::try_from(path)?;
        // supported extension is only epub at the moment, just going with the epub extraction method
    }

    todo!()
}
