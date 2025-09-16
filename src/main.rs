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
}
