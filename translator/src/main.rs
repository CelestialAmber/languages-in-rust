// Code taken straight from
// https://github.com/AbdelStark/anthropic-rs/blob/main/examples/basic-completion/src/main.rs
// I just changed the prompt and added input.

use std::error::Error;

use anthropic::client::Client;
use anthropic::config::AnthropicConfig;
use anthropic::types::CompleteRequestBuilder;
use anthropic::{AI_PROMPT, HUMAN_PROMPT};
use dotenv::dotenv;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger.
    env_logger::init();

    // Load the environment variables from the .env file.
    dotenv().ok();

    // Build from configuration.
    let cfg = AnthropicConfig::new()?;
    let client = Client::try_from(cfg)?;

    let text = io::read_to_string(io::stdin())?;
    let prompt = format!("{HUMAN_PROMPT}\
    Translate the following text to Rust. If there is no straightforward way to do so, do your best anyway.
    Just answer with the straight translation. Do NOT answer with any comments, preludes, explanations, or anything that isn't the translation.
    Stay faithful to the original content while producing correct and beautiful Rust code.
    
    Here is the text to translate:
    {text}\
    {AI_PROMPT}");

    let complete_request = CompleteRequestBuilder::default()
        .prompt(prompt)
        .model("claude-instant-1".to_string())
        .max_tokens_to_sample(256usize)
        .stream(false)
        .stop_sequences(vec![HUMAN_PROMPT.to_string()])
        .build()?;

    // Send a completion request.
    let complete_response = client.complete(complete_request).await?;
    println!("{}", complete_response.completion);
    Ok(())
}
