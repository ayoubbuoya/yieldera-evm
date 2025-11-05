use rig::{agent::Agent, providers::gemini::completion::CompletionModel};

use crate::core::init::init_ai_agent;

pub struct AppState {
    pub ai_agent: Agent<CompletionModel>,
}

impl AppState {
    pub async fn new() -> Self {
        // Initialize the AI agent
        let ai_agent = init_ai_agent()
            .await
            .expect("Failed to initialize AI agent");

        Self { ai_agent }
    }
}
