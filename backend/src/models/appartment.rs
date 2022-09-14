use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{broker::Broker, currency::Money, status::Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Apartment {
    pub starting_price: Money,
    pub broker: Broker,
    pub images: Vec<String>,
    pub description: String,
    pub address: String,
    pub status: Status,
    pub size: Option<f32>,
    pub area_name: String,
    pub fee: Option<Money>,
    pub number_of_rooms: f32,
    pub published_date: DateTime<Utc>,
}
