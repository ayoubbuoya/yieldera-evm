use anyhow::Result;
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
use serde_json::Value;
use tracing::info;

use crate::{
    config::{COINGECKO_NETWORK_ID, CONFIG},
    types::CoingeckoOhlcvRes,
};

pub async fn get_pool_ohlcv_data(pool_address: &str) -> Result<CoingeckoOhlcvRes> {
    let url = format!(
        "https://api.coingecko.com/api/v3/onchain/networks/{}/pools/{}/ohlcv/day?token=base&currency=token&limit=1000",
        COINGECKO_NETWORK_ID, pool_address
    );

    let coingecko_api_key = &CONFIG.coingecko_api_key;

    // Set up headers
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        "x-cg-demo-api-key",
        HeaderValue::from_str(&coingecko_api_key)?,
    );

    // Make request
    let client = reqwest::Client::new();

    let response = client.get(url).headers(headers).send().await?;

    let ohlcv_data_res: Value = response.json().await?;

    let ohlcv_data: CoingeckoOhlcvRes = serde_json::from_value(ohlcv_data_res)?;

    info!(
        "Coingecko data fetched successfully: {:?}",
        ohlcv_data.data.attributes.ohlcv_list.len()
    );

    Ok(ohlcv_data)
}
