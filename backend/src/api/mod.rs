use std::str::FromStr;

use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    core::coingecko::get_pool_ohlcv_data, state::AppState, strategies::{Strategy, narrow::NarrowStrategy, wide::WideStrategy}, types::{CoingeckoOhlcvRes, RangeSuggestion, StrategyType}
};

#[utoipa::path(
        responses(
            (status = 200, description = "Home page", body = String),
        )
    )]
#[get("/")]
async fn get_index_service() -> impl Responder {
    HttpResponse::Ok().body("UP")
}

#[utoipa::path(
    responses(
        (status = 200, description = "Health check", body = String),
    )
)]
#[get("/health")]
async fn get_health_service() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[utoipa::path(
    responses(
        (status = 200, description = "Coingecko data", body = CoingeckoOhlcvRes),
    )
)]
#[get("/coingecko/{pool_address}")]
async fn get_pool_coingecko_data(pool_address: web::Path<String>) -> impl Responder {
    let coingecko_data = get_pool_ohlcv_data(&pool_address).await;

    match coingecko_data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Suggested liquidity range", body = RangeSuggestion),
    )
)]
#[get("/pools/{pool_address}/liquidity/{strategy}/suggest")]
async fn suggest_liquidity_range(
    app_state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (pool_address, strategy_str) = path.into_inner();

    // Validate strategy type
    let strategy = match StrategyType::from_str(&strategy_str) {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::BadRequest()
                .body("Invalid strategy type. Use 'narrow' or 'wide'.");
        }
    };

    // use the strategy trait to suggest liquidity range
    let suggestion = match strategy {
        StrategyType::Narrow => {
            NarrowStrategy
                .suggest_price_range(&app_state, &pool_address)
                .await
        }
        StrategyType::Wide => {
            WideStrategy
                .suggest_price_range(&app_state, &pool_address)
                .await
        }
    };

    match suggestion {
        Ok(suggestion) => HttpResponse::Ok().json(suggestion),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
