use std::{thread, time::Duration};

use crate::models::{
    appartment::Apartment,
    broker::Broker,
    currency::{Currency, Money},
    status::Status,
};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

const ERIKOLSSON_BASE_ADDRESS: &str = "https://www.erikolsson.se";
const AREA_ID: &str = "23-Göteborg, Västra Götalands län";

pub async fn get_available_apartments() -> Vec<ErikOlssonPropertyJson> {
    let mut erikolsson_apartments: Vec<ErikOlssonPropertyJson> = vec![];
    let mut page = 0;

    loop {
        let mut res = reqwest::get(get_url(page))
            .await
            .expect("Failed to make get request")
            .json::<ErikOlssonResponseJson>()
            .await
            .expect("Failed to parse json");

        erikolsson_apartments.append(&mut res.properties);

        println!(
            "Retrieved {} out of {} apartments (page {}), continue? {}",
            erikolsson_apartments.len(),
            res.hits,
            page,
            res.should_render_show_more_button
        );
        if !res.should_render_show_more_button {
            break;
        } else {
            page = page + 1
        }

        thread::sleep(Duration::from_millis(1500));
    }

    erikolsson_apartments
}

fn get_url(page: usize) -> String {
    format!(
        "{}/api/search?areaIds={}&page={}&propertyType=lägenhet&minRooms=2&minArea=45&maxPrice=3300000&sortOrder=&internalOnly=true",
        ERIKOLSSON_BASE_ADDRESS, AREA_ID, page
    )
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ErikOlssonResponseJson {
    should_render_show_more_button: bool,
    hits: u32,
    properties: Vec<ErikOlssonPropertyJson>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErikOlssonPropertyJson {
    vitec_object_id: String,
    start_price: u32,
    main_image_url: String,
    city: String,
    area_name: String,
    address: String,
    price: Option<String>,
    description: String,
    published_date: String,
    url: String,
    fee: Option<String>,
    is_apartment: bool,
    number_of_rooms: f32,
    hide_price: bool,
    show_as_coming: bool,
    is_new_production: bool,
}

impl Into<Apartment> for ErikOlssonPropertyJson {
    fn into(self) -> Apartment {
        Apartment {
            starting_price: Money {
                amount: self.start_price,
                currency: Currency::Sek,
            },
            status: self.get_status(),
            fee: self.get_fee(),
            published_date: self.get_published_date(),
            broker: Broker::ErikOlsson,
            images: vec![self.main_image_url],
            description: self.description,
            address: self.address,
            area_name: self.area_name,
            number_of_rooms: self.number_of_rooms,
        }
    }
}

impl ErikOlssonPropertyJson {
    fn get_status(&self) -> Status {
        if self.is_new_production {
            Status::NewProduction
        } else if self.show_as_coming {
            Status::Upcomming
        } else {
            Status::ForSale
        }
    }

    fn get_fee(&self) -> Option<Money> {
        match self.fee.clone() {
            Some(s) => {
                let stripped = s
                    .strip_suffix(" kr/månad")
                    .expect("Failed to strip kr/month ending");

                Some(Money {
                    amount: stripped
                        .parse::<u32>()
                        .expect(&format!("Failed to parse u32 from fee {}", s)),
                    currency: Currency::Sek,
                })
            }
            None => None,
        }
    }

    fn get_published_date(&self) -> DateTime<Utc> {
        Utc.from_local_datetime(
            &NaiveDateTime::parse_from_str(&self.published_date, "%Y-%m-%dT%H:%M:%S").expect(
                &format!(
                    "Failed to parse DateTime for published date ({})",
                    self.published_date
                ),
            ),
        )
        .unwrap()
    }
}
