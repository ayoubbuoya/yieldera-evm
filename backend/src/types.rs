use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoingeckoOhlcvRes {
    data: CoingeckoResData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoingeckoResData {
    pub id: String,
    pub attributes: CoingeckoResDataAttributes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoingeckoResDataAttributes {
    pub ohlcv_list: Vec<OhlcvEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OhlcvEntry(
    i64, // timestamp (UNIX)
    f64, // open
    f64, // high
    f64, // low
    f64, // close
    f64, // volume
);
