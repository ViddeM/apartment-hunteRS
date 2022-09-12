use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    ForSale,
    Upcomming,
    NewProduction,
}
