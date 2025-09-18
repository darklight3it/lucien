use std::path::Path;

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode};

use crate::apis::models::EbookMetadata;
use crate::apis::openlibrary::fetch_ebook_metadata_by_isbn;
use crate::ebook::ebook_metadata::get_isbn;
use crate::ebook::models::Isbn;

#[macro_use]
extern crate log;
extern crate simplelog;
mod apis;
mod app_config;
mod ebook;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load app configuration
    let _config = match app_config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            panic!("Error loading config:{}", e);
        }
    };

    setup_logger();

    let path = Path::new("tests/resources/test_book.epub");

    let isbn: Isbn = match get_isbn(path) {
        Ok(i) => {
            debug!("the isbn is: {}", i);
            i
        }
        Err(e) => {
            panic!("Error getting isbn: {}", e)
        }
    };

    let _metas: EbookMetadata = match fetch_ebook_metadata_by_isbn(&isbn).await {
        Ok(i) => {
            debug!("the downloaded metas are: {}", i);
            i
        }
        Err(e) => {
            panic!("Error getting metas: {}", e)
        }
    };

    Ok(())
}

fn setup_logger() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
#[cfg(test)]
#[test]
fn my_test() {
    // Enabling logging for testing
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        TermLogger::init(
            LevelFilter::Info,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
        .unwrap();
    });
}
