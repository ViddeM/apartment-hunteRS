use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    error::ApartmentResult,
    models::{
        appartment::Apartment,
        broker::Broker,
        currency::{Currency, Money},
        status::Status,
    },
};

const LUNDIN_BASE_ADDRESS: &str = "https://www.lundin.se/lista/tillsalu";

pub async fn get_available_apartments() -> ApartmentResult<Vec<LundinPropertyJson>> {
    let res = reqwest::get(LUNDIN_BASE_ADDRESS).await?.text().await?;
    let apartment_json_regex = Regex::new(r"\[\{.*\}\]")?;

    let match_location = apartment_json_regex
        .find(&res)
        .expect("Failed to find a json match in Lundin json");

    let lundin_json: Vec<LundinPropertyJson> = serde_json::from_str(match_location.as_str())?;

    println!(
        "Retrieved {} apartments from lundin, {} matched my criterias",
        lundin_json.len(),
        lundin_json
            .into_iter()
            .filter(|a| a.is_up_coming == false
                && a.is_newprod == false
                && a.price < 3300000
                && a.rent < 4500
                && a.rooms >= 2.0
                && a.area >= 45.0)
            .collect::<Vec<LundinPropertyJson>>()
            .len()
    );

    Ok(vec![])
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]

pub struct LundinPropertyJson {
    image_url: Option<String>,
    slide_image_url: Option<String>,
    city: String,
    address: String,
    price: u32,
    rent: u32,
    area: f32,
    guid: String,
    status: String,
    location: Option<String>,
    date_string: String,
    parent_location: String,
    is_up_coming: bool,
    is_sold: bool,
    is_exclusive: bool,
    is_highlight: bool,
    is_newprod: bool,
    heading: String,
    #[serde(rename = "Descr")]
    description: String,
    #[serde(rename = "DescrUrl")]
    descr_url: String,
    rooms: f32,
    buildyear: Option<u32>,
    list_heading: String,
    object_type: String,
    list_simple_heading: String,
}

impl Into<Apartment> for LundinPropertyJson {
    fn into(self) -> Apartment {
        Apartment {
            images: self.get_images(),
            status: self.get_status(),
            published_date: self.get_published_date(),
            starting_price: Money {
                amount: self.price,
                currency: Currency::Sek,
            },
            broker: Broker::Lundin,
            description: self.description,
            address: self.address,
            area_name: self.location.unwrap_or(String::new()),
            fee: Some(Money {
                amount: self.rent,
                currency: Currency::Sek,
            }),
            number_of_rooms: self.rooms,
            size: Some(self.area),
        }
    }
}

impl LundinPropertyJson {
    fn get_images(&self) -> Vec<String> {
        let mut images = vec![];
        if let Some(url) = self.image_url.clone() {
            images.push(url)
        }
        if let Some(url) = self.slide_image_url.clone() {
            images.push(url)
        }

        return images;
    }

    fn get_status(&self) -> Status {
        if self.is_newprod {
            Status::NewProduction
        } else if self.is_up_coming {
            Status::Upcoming
        } else {
            Status::ForSale
        }
    }

    fn get_published_date(&self) -> DateTime<Utc> {
        Utc.from_local_datetime(
            &NaiveDateTime::parse_from_str(&self.date_string, "%Y-%m-%dT%H:%M:%S").expect(
                &format!(
                    "Failed to parse DateTime for date string ({})",
                    self.date_string
                ),
            ),
        )
        .unwrap()
    }
}
