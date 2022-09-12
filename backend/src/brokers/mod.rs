use serde::{Deserialize, Serialize};

use crate::models::appartment::Apartment;

use self::erikolsson::ErikOlssonPropertyJson;

mod erikolsson;

#[derive(Serialize, Deserialize, Clone)]
pub enum BrokerApartment {
    ErikOlsson(ErikOlssonPropertyJson),
}

pub async fn get_apartments() -> Vec<BrokerApartment> {
    erikolsson::get_available_apartments()
        .await
        .into_iter()
        .map(|a| BrokerApartment::ErikOlsson(a))
        .collect()
}

impl Into<Apartment> for BrokerApartment {
    fn into(self) -> Apartment {
        match self {
            BrokerApartment::ErikOlsson(a) => a.into(),
        }
    }
}
