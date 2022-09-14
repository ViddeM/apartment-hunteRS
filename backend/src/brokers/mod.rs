use serde::{Deserialize, Serialize};

use crate::{error::ApartmentResult, models::appartment::Apartment};

use self::erikolsson::ErikOlssonPropertyJson;

mod erikolsson;
mod lundin;

#[derive(Serialize, Deserialize, Clone)]
pub enum BrokerApartment {
    ErikOlsson(ErikOlssonPropertyJson),
}

pub async fn get_apartments() -> ApartmentResult<Vec<BrokerApartment>> {
    let erikolsson_apartments = erikolsson::get_available_apartments()
        .await?
        .into_iter()
        .map(|a| BrokerApartment::ErikOlsson(a))
        .collect();

    let lundin_apartments = lundin::get_available_apartments().await?;

    Ok(erikolsson_apartments)
}

impl Into<Apartment> for BrokerApartment {
    fn into(self) -> Apartment {
        match self {
            BrokerApartment::ErikOlsson(a) => a.into(),
        }
    }
}
