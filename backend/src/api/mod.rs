use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{core::coingecko::get_pool_ohlcv_data, types::CoingeckoOhlcvRes};

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
