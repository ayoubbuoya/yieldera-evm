pub mod narrow;
pub mod wide;

use actix_web::web;
use anyhow::Result;

use crate::{state::AppState, types::RangeSuggestion};

pub trait Strategy {
    async fn suggest_price_range(
        &self,
        app_state: &web::Data<AppState>,
        pool_address: &str,
    ) -> Result<RangeSuggestion>;
}
