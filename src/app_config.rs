use config::Config;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub model_path: String,
    pub prompt: String,
    pub metadata_index_path: PathBuf,
}

pub fn load_config() -> Result<AppConfig, String> {
    let config = Config::builder()
        .add_source(config::File::with_name("./Config.toml"))
        .add_source(config::Environment::with_prefix("LUCIEN"))
        .build()
        .map_err(|e| format!("Failed to load Config.toml or environment variables: {}", e))?;

    // Try to deserialize into AppConfig
    let app_config = config
        .try_deserialize::<AppConfig>()
        .map_err(|e| format!("Failed to deserialize configuration: {}", e))?;

    Ok(app_config)
}
