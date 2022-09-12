#![forbid(unsafe_code)]
#[macro_use]
extern crate rocket;

mod api;
pub mod brokers;
pub mod config;
pub mod error;
mod models;
pub mod services;

use config::Config;
use mobc_redis::{mobc::Pool, redis::Client, RedisConnectionManager};

const MAX_REDIS_CONNECTONS: u64 = 20;

#[launch]
async fn rocket() -> _ {
    // Load
    let config = Config::new().expect("Failed to load config");

    // Setup Redis cache
    let redis_client = Client::open(config.redis_url.clone()).expect(&format!(
        "Failed to connect to redis on URL {}",
        config.redis_url
    ));
    let redis_manager = RedisConnectionManager::new(redis_client);
    let redis_pool = Pool::builder()
        .max_open(MAX_REDIS_CONNECTONS)
        .build(redis_manager);

    // Test redis connection
    redis_pool
        .get()
        .await
        .expect("Test connection to redis pool failed");

    rocket::build()
        .mount("/api", routes![api::get_apartments::get_apartments])
        .manage(redis_pool)
}
