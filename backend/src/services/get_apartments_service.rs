use chrono::{DateTime, Utc};
use mobc_redis::{mobc::Pool, RedisConnectionManager};
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{
    brokers::{self, BrokerApartment},
    error::ApartmentResult,
    models::appartment::Apartment,
};

use super::redis_service;

const BROKER_APARTMENTS_KEY: &str = "broker_apartments";

#[derive(Serialize, Deserialize, Clone)]
pub struct RedisEntry {
    pub insert_timestamp: DateTime<Utc>,
    pub broker_apartments: Vec<BrokerApartment>,
}

pub async fn get_all_apartments(
    redis_pool: &State<Pool<RedisConnectionManager>>,
) -> ApartmentResult<Vec<Apartment>> {
    let apps = match get_cached_apartments(redis_pool).await? {
        Some(entry) => entry.broker_apartments,
        None => {
            println!("No apartments in cache");
            vec![]
        }
    };

    Ok(apps.into_iter().map(|a| a.into()).collect())
}

pub async fn get_cached_apartments(
    redis_pool: &Pool<RedisConnectionManager>,
) -> ApartmentResult<Option<RedisEntry>> {
    redis_service::redis_get_option::<RedisEntry>(redis_pool, BROKER_APARTMENTS_KEY).await
}

pub async fn refresh_apartments_cache(
    redis_pool: &Pool<RedisConnectionManager>,
) -> ApartmentResult<()> {
    let broker_apartments = brokers::get_apartments().await?;

    let redis_entry = RedisEntry {
        insert_timestamp: Utc::now(),
        broker_apartments: broker_apartments.clone(),
    };

    redis_service::redis_set::<RedisEntry>(
        redis_pool,
        BROKER_APARTMENTS_KEY.to_string(),
        redis_entry,
    )
    .await
    .expect("Failed to set redis value");

    Ok(())
}
