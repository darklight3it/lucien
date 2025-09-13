use llama_cpp::standard_sampler::StandardSampler;
use llama_cpp::{LlamaModel, LlamaParams, SessionParams};
use std::io::{self, Write};
mod app_config;
mod ebook;
mod llm;

fn main() {
    // load app configuration
    let settings = match app_config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            panic!("Error loading config:{}", e);
        }
    };

    let model = match LlamaModel::load_from_file(settings.model_path, LlamaParams::default()) {
        Ok(m) => {
            println!("✅ Model loaded successfully");
            m
        }
        Err(e) => {
            eprintln!("❌ Failed to load model: {:?}", e);
            return;
        }
    };

    // A `LlamaModel` holds the weights shared across many _sessions_; while your model may be
    // several gigabytes large, a session is typically a few dozen to a hundred megabytes!
    let mut ctx = model
        .create_session(SessionParams::default())
        .expect("Failed to create session");

    // You can feed anything that implements `AsRef<[u8]>` into the model's context.
    ctx.advance_context("asd").unwrap();

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

    println!("Hello, world!");
}
