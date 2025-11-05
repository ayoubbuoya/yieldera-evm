use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CoingeckoOhlcvRes {
    pub data: CoingeckoResData,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CoingeckoResData {
    pub id: String,
    pub attributes: CoingeckoResDataAttributes,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CoingeckoResDataAttributes {
    pub ohlcv_list: Vec<OhlcvEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct OhlcvEntry(
    i64, // timestamp (UNIX)
    f64, // open
    f64, // high
    f64, // low
    f64, // close
    f64, // volume
);

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RangeSuggestion {
    pub up_price: f64,
    pub down_price: f64,
    pub confidence: f64,
    pub reason: String,
    pub strategy: StrategyType,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub enum StrategyType {
    Narrow,
    Wide,
}

// Implment from str for StrategyType
impl std::str::FromStr for StrategyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "narrow" => Ok(StrategyType::Narrow),
            "wide" => Ok(StrategyType::Wide),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct AIResponse {
    pub low_price: f64,
    pub upper_price: f64,
    pub confidence: f64,
    pub reason: String,
}
