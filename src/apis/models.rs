use std::fmt::{Display, write};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OpenLibraryResponse {
    pub docs: Vec<EbookMetadata>,
}

// Optional: parse JSON into a struct
#[derive(Deserialize, Debug)]
pub struct EbookMetadata {
    pub title: String,
    pub author_name: Vec<String>,
}

impl Display for EbookMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.title, self.author_name.join(","))
    }
}
