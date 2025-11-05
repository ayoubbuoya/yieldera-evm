use anyhow::Result;
use rig::{
    agent::Agent,
    client::CompletionClient,
    providers::gemini::{
        self,
        completion::{
            CompletionModel,
            gemini_api_types::{AdditionalParameters, GenerationConfig, Schema},
        },
    },
};

pub async fn init_ai_agent() -> Result<Agent<CompletionModel>> {
    // Initialize the Google Gemini client
    let client = gemini::Client::from_env();

    // Configure additional parameters for the completion
    let schema_json = serde_json::json!({
        "type": "object",
        "properties": {
            "low_price": {
                "type": "number"
            },
            "upper_price": {
                "type": "number"
            },
            "confidence": {
                "type": "number"
            },
            "reason": {
                "type": "string"
            }
        },
        "required": [
            "low_price",
            "upper_price",
            "confidence",
            "reason"
        ],
        "propertyOrdering": [
            "low_price",
            "upper_price",
            "confidence",
            "reason"
        ]
    });

    let schema: Schema = serde_json::from_value(schema_json)?;

    let gen_cfg = GenerationConfig {
        response_schema: Some(schema),
        ..Default::default()
    };
    let cfg = AdditionalParameters::default().with_config(gen_cfg);

    // Create agent with a single context prompt
    let agent = client
        .agent("gemini-flash-latest")
        .preamble("You are a liquidity manager AI assistant. Your goal is to help users optimize their yield farming strategies on EVM-compatible blockchains by suggesting the best price range to provide liquidity based on current market conditions and historical data (data will be provided to you on the prompt by coingecko).")
        .temperature(0.05)
        // The `GenerationConfig` utility struct helps construct a typesafe `additional_params`
        .additional_params(serde_json::to_value(cfg)?) // Unwrap the Result to get the Value
        .build();

    tracing::info!("AI Agent initialized successfully.");

    Ok(agent)
}
