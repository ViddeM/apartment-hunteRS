use mobc_redis::{mobc::Pool, redis::AsyncCommands, RedisConnectionManager};
use rocket::{serde::DeserializeOwned, State};
use serde::Serialize;

use crate::error::ApartmentResult;

pub async fn redis_get<T: DeserializeOwned>(
    redis_pool: &Pool<RedisConnectionManager>,
    key: &str,
) -> ApartmentResult<T> {
    let mut redis_conn = redis_pool.get().await?;
    let raw_result: String = redis_conn.get(key).await?;

    Ok(serde_json::from_str::<T>(&raw_result)?)
}

pub async fn redis_get_option<T: DeserializeOwned>(
    redis_pool: &Pool<RedisConnectionManager>,
    key: &str,
) -> ApartmentResult<Option<T>> {
    let mut redis_conn = redis_pool.get().await?;
    let raw_result: Option<String> = redis_conn.get(key).await?;
    match raw_result {
        Some(s) => Ok(Some(serde_json::from_str::<T>(&s)?)),
        None => Ok(None),
    }
}

pub async fn redis_set<T>(
    redis_pool: &Pool<RedisConnectionManager>,
    key: String,
    value: T,
) -> ApartmentResult<()>
where
    T: Serialize,
{
    let mut redis_conn = redis_pool.get().await?;
    redis_conn
        .set::<String, String, String>(key, serde_json::to_string(&value)?)
        .await?;
    Ok(())
}
