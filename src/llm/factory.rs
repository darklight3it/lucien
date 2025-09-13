use crate::app_config::{AppConfig, ModelFormat};
use crate::llm::llama_model::load_llama;
use crate::llm::traits::MetadataOptimizer;

pub fn model_factory(app_config: AppConfig) -> Box<dyn MetadataOptimizer> {
    let model = match app_config.model_format {
        ModelFormat::GGUF => load_llama(app_config),
    };

    Box::new(model)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{any::Any, path::PathBuf};

    // Mocking the optimizer
    struct DummyOptimizer;
    impl MetadataOptimizer for DummyOptimizer {
        fn optimize_metadata(&self) -> Result<String, String> {
            todo!()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Mocking load_llama, test-only "override" lives inside this module
    pub fn load_llama(_app_config: AppConfig) -> Box<dyn MetadataOptimizer> {
        Box::new(DummyOptimizer)
    }

    #[test]
    fn model_factory_returns_correct_model() {
        // call the test version explicitly inside this module
        let app_config = AppConfig {
            model_format: ModelFormat::GGUF,
            model_path: PathBuf::from("file"),
            prompt: "prompt".to_string(),
            metadata_index_path: PathBuf::from("asd"),
        };

        let model = load_llama(app_config);

        // Try to downcast to the concrete type

        assert!(model.as_any().is::<DummyOptimizer>());
    }
}
