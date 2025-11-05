use actix_web::web;
use anyhow::Result;
use rig::completion::Prompt;

use crate::{
    core::coingecko,
    state::AppState,
    strategies::Strategy,
    types::{AIResponse, RangeSuggestion, StrategyType},
    utils::extract_json_from_markdown,
};

pub struct NarrowStrategy;

impl Strategy for NarrowStrategy {
    async fn suggest_price_range(
        &self,
        app_state: &web::Data<AppState>,
        pool_address: &str,
    ) -> Result<RangeSuggestion> {
        let ai_agent = &app_state.ai_agent;

        // Fetch coingecko data for the pool
        let coingecko_ohlcv_data = coingecko::get_pool_ohlcv_data(pool_address).await?;

        // Use the AI agent to analyze data and suggest a price range
        let analysis_prompt = format!(
            "Use narrow  liquidity  position strategy. Analyze the following OHLCV data and suggest a liquidity range for the pool '{}'. Return the lower price and the upper price, the reason for the suggestion and the confidence level. Return only Json object that respect this Schema: {{\"low_price\": number, \"upper_price\": number, \"confidence\": number, \"reason\": string}}. The Coingecko History Data: {:?}",
            pool_address, coingecko_ohlcv_data
        );

        tracing::debug!("Prompt sent to AI agent...");

        let ai_response_str = ai_agent.prompt(analysis_prompt).await?;

        tracing::info!("AI Response: {:?}", ai_response_str);

        let formatted_md_response = extract_json_from_markdown(&ai_response_str);

        let ai_response: AIResponse = serde_json::from_str(&formatted_md_response)?;

        tracing::info!("AI Response: {:?}", ai_response);

        Ok(RangeSuggestion {
            up_price: ai_response.upper_price,
            down_price: ai_response.low_price,
            confidence: ai_response.confidence,
            reason: ai_response.reason,
            strategy: StrategyType::Narrow,
        })
    }
}
