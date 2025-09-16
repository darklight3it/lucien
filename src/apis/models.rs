use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OpenLibraryResponse {
    pub docs: Vec<Book>,
}

// Optional: parse JSON into a struct
#[derive(Deserialize, Debug)]
pub struct Book {
    pub title: String,
    pub authors: Vec<AuthorRef>,
}

#[derive(Deserialize, Debug)]
pub struct AuthorRef {
    pub key: String,
}
