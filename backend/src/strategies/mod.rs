pub mod narrow;
pub mod wide;

use anyhow::Result;

use crate::types::RangeSuggestion;

pub trait Strategy {
    async fn suggest_price_range(
        &self,
        pool_address: &str,
        strategy: &str,
    ) -> Result<RangeSuggestion>;
}
