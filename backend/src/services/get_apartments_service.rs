use mobc_redis::{mobc::Pool, RedisConnectionManager};
use rocket::State;

use crate::{
    brokers::{self, BrokerApartment},
    error::ApartmentResult,
    models::appartment::Apartment,
};

use super::redis_service;

const BROKER_APARTMENTS_KEY: &str = "broker_apartments";

pub async fn get_all_apartments(
    redis_pool: &State<Pool<RedisConnectionManager>>,
) -> ApartmentResult<Vec<Apartment>> {
    let apartments = reget_apartments(redis_pool)?;
}

async fn reget_apartments(
    redis_pool: &State<Pool<RedisConnectionManager>>,
) -> ApartmentResult<Vec<Apartment>> {
    let broker_apartments = brokers::get_apartments().await;
    redis_service::redis_set::<Vec<BrokerApartment>>(
        redis_pool,
        BROKER_APARTMENTS_KEY.to_string(),
        broker_apartments.clone(),
    )
    .await?;

    Ok(broker_apartments.into_iter().map(|a| a.into()).collect())
}
