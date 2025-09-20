use crate::apis::models::OpenLibraryResponse;
use crate::ebook::models::Isbn;

use super::errors::ClientError;
use super::models::EbookMetadata;

pub async fn fetch_ebook_metadata_by_isbn(isbn: &Isbn) -> Result<EbookMetadata, ClientError> {
    // Example: GET request to Open Library API
    let url = format!("https://openlibrary.org/search.json?isbn={}", isbn.get_id());

    // ? operator is returning the response and bubbling the error to the caller
    // internalerror is automatically deriving the Error from ClientError enum
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;

    debug!("Raw:{}", body);

    let response: OpenLibraryResponse = serde_json::from_str(&body)?;

    // Openlibrary Search API return 200 even if the result bears 0 books
    // in this case we are searching for ISBN so either we get 1 result or nothing
    if let Some(doc) = response.docs.into_iter().next() {
        Ok(doc)
    } else {
        Err(ClientError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Book not found".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_fetch_book_by_isbn() {
        let isbn = &Isbn::new(&"9780141439600".to_string()).unwrap(); // Pride and Prejudice
        let book: EbookMetadata = fetch_ebook_metadata_by_isbn(isbn).await.unwrap();

        assert_eq!(book.title, "A Tale of Two Cities");
        let authors = book.author_name;
        assert!(!authors.is_empty());
        assert_eq!("Charles Dickens", authors.iter().next().unwrap());
    }
}
