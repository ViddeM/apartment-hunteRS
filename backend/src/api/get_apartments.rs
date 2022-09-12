use mobc_redis::{
    mobc::{self},
    RedisConnectionManager,
};
use rocket::State;

use crate::services::get_apartments_service;

#[get("/apartments")]
pub async fn get_apartments(redis_pool: &State<mobc::Pool<RedisConnectionManager>>) {
    get_apartments_service::get_all_apartments(redis_pool)
        .await
        .expect("Failed to get apartments");
}
