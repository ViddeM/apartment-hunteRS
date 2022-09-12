use chrono::{Duration, Utc};
use mobc_redis::{mobc::Pool, RedisConnectionManager};
use tokio::time;

use crate::{error::ApartmentResult, services::get_apartments_service};

// 30 minutes
const SECONDS_BETWEEN_TASK_CHECKS: u64 = 60 * 30;

const HOURS_BETWEEN_DATA_UPDATES: i64 = 12;

pub async fn run_background_tasks(redis_pool: Pool<RedisConnectionManager>) {
    let time_between_tasks = std::time::Duration::from_secs(SECONDS_BETWEEN_TASK_CHECKS);

    // Keep checking 'forever'
    loop {
        update_apartment_cache(&redis_pool)
            .await
            .expect("Failed to update apartment cache");
        time::sleep(time_between_tasks).await;
    }
}

async fn update_apartment_cache(redis_pool: &Pool<RedisConnectionManager>) -> ApartmentResult<()> {
    println!("Checking if we should update the apartment cache");
    let duration_between_apartment_refresh = Duration::hours(HOURS_BETWEEN_DATA_UPDATES);

    let entry = get_apartments_service::get_cached_apartments(redis_pool).await?;

    let should_refresh = match entry {
        Some(e) => {
            let now = Utc::now();
            if now - e.insert_timestamp > duration_between_apartment_refresh {
                println!("Cached data is outdated, refreshing...");
                true
            } else {
                false
            }
        }
        None => {
            println!("No cached data found, retrieving...");
            true
        }
    };

    if should_refresh {
        println!("Begin refreshing apartments...");
        get_apartments_service::refresh_apartments_cache(redis_pool).await?;
        println!("Finished refreshing apartments cache!");
    }

    Ok(())
}
