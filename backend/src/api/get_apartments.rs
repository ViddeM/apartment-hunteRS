use mobc_redis::{
    mobc::{self},
    RedisConnectionManager,
};
use rocket::{serde::json::Json, State};

use crate::{models::appartment::Apartment, services::get_apartments_service};

#[get("/apartments")]
pub async fn get_apartments(
    redis_pool: &State<mobc::Pool<RedisConnectionManager>>,
) -> Json<Vec<Apartment>> {
    Json(
        get_apartments_service::get_all_apartments(redis_pool)
            .await
            .expect("Failed to get apartments"),
    )
}
