use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OpenLibraryResponse {
    pub docs: Vec<Book>,
}

// Optional: parse JSON into a struct
#[derive(Deserialize, Debug)]
pub struct Book {
    pub title: String,
    pub author_name: Vec<String>,
}
