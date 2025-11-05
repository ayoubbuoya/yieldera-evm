use anyhow::Result;
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};

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

    let ohlcv_data: CoingeckoOhlcvRes = response.json().await?;

    Ok(ohlcv_data)
}
