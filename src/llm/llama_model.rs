use std::any::Any;

use crate::app_config::AppConfig;
use crate::llm::traits::MetadataOptimizer;
use llama_cpp::{LlamaModel, LlamaParams};

pub fn load_llama(app_config: AppConfig) -> LlamaModel {
    let model = match LlamaModel::load_from_file(app_config.model_path, LlamaParams::default()) {
        Ok(m) => {
            println!("✅ Model loaded successfully");
            m
        }
        Err(e) => {
            panic!("❌ Failed to load model: {:?}", e)
        }
    };

    model
}

impl MetadataOptimizer for LlamaModel {
    fn optimize_metadata(&self) -> Result<String, String> {
        Err("Not yet implemented".to_string())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llama_cpp::LlamaLoadError;
    use mockall::*;

    mock! {
      LlamaModel
    }

    #[test]
    fn optimize_data_should_do_something() {}
}
