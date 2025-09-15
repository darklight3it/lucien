use crate::llm::factory::model_factory;
mod app_config;
mod ebook;
mod llm;

fn main() {
    // load app configuration
    let config = match app_config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            panic!("Error loading config:{}", e);
        }
    };

    // using a factory to instantiate the model
    let model = match model_factory(&config) {
        Ok(m) => {
            println!("✅ Model loaded successfully");
            m
        }
        Err(e) => {
            panic!("❗️ Failed to load model: {:?}", e)
        }
    };

    // using the model to get an optimized json
    let metadata = "[
        {
            \"title\": \"Dracula\",
            \"author\": \"Davide Melfi\",
            \"first_published\": \"\"
        },
        {
            \"title\": \"Frankenstein\",
            \"author\": \"Davide Melfi\",
            \"first_published\": \"\"
        }
    ]";
    let result = model.optimize_metadata(&metadata.to_string(), &config);
}
