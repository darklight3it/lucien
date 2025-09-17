use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode};

#[macro_use]
extern crate log;
extern crate simplelog;
mod apis;
mod app_config;
mod ebook;

fn main() {
    // load app configuration
    let config = match app_config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            panic!("Error loading config:{}", e);
        }
    };

    setup_logger();
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
