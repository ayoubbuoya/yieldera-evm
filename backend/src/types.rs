use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CoingeckoOhlcvRes {
    data: CoingeckoResData,
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
