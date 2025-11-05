use anyhow::Result;
use rig::{
    agent::Agent,
    client::CompletionClient,
    providers::gemini::{
        self,
        completion::{
            CompletionModel,
            gemini_api_types::{AdditionalParameters, GenerationConfig},
        },
    },
};

pub async fn init_ai_agent() -> Result<Agent<CompletionModel>> {
    // Initialize the Google Gemini client
    let client = gemini::Client::from_env();

    // Configure additional parameters for the completion
    let gen_cfg = GenerationConfig {
        ..Default::default()
    };
    let cfg = AdditionalParameters::default().with_config(gen_cfg);

    // Create agent with a single context prompt
    let agent = client
        .agent("gemini-2.5-flash")
        .preamble("You are a liquidity manager AI assistant. Your goal is to help users optimize their yield farming strategies on EVM-compatible blockchains by suggesting the best price range to provide liquidity based on current market conditions and historical data (data will be provided to you on the prompt by coingecko).")
        .temperature(0.2)
        // The `GenerationConfig` utility struct helps construct a typesafe `additional_params`
        .additional_params(serde_json::to_value(cfg)?) // Unwrap the Result to get the Value
        .build();

    tracing::info!("AI Agent initialized successfully.");

    Ok(agent)
}
