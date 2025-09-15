use std::any::Any;
use std::io::{self, Write};

use crate::app_config::AppConfig;
use crate::llm::traits::MetadataOptimizer;
use llama_cpp::{
    LlamaLoadError, LlamaModel, LlamaParams, SessionParams, standard_sampler::StandardSampler,
};

pub fn load_llama(app_config: &AppConfig) -> Result<LlamaModel, LlamaLoadError> {
    return LlamaModel::load_from_file(&app_config.model_path, LlamaParams::default());
}

impl MetadataOptimizer for LlamaModel {
    fn optimize_metadata(
        &self,
        metadata: &String,
        app_config: &AppConfig,
    ) -> Result<String, String> {
        let mut ctx = self
            .create_session(SessionParams::default())
            .expect("Failed to create session");

        // You can feed anything that implements `AsRef<[u8]>` into the model's context.
        let feed = format!("{}{}", app_config.prompt, metadata);
        ctx.advance_context(&feed).unwrap();

        // LLMs are typically used to predict the next word in a sequence. Let's generate some tokens!
        let max_tokens = 1024;
        let mut decoded_tokens: i32 = 0;

        // `ctx.start_completing_with` creates a worker thread that generates tokens. When the completion
        // handle is dropped, tokens stop generating!
        let completions = ctx
            .start_completing_with(StandardSampler::default(), 1024)
            .unwrap()
            .into_strings();

        for completion in completions {
            print!("{completion}");
            let _ = io::stdout().flush();

            decoded_tokens += 1;

            if decoded_tokens > max_tokens {
                break;
            }
        }
        Ok("Completed".to_string())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
