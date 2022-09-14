use crate::error::ApartmentResult;

const LUNDIN_BASE_ADDRESS: &str = "https://www.lundin.se/lista/tillsalu";

pub async fn get_available_apartments() -> ApartmentResult<Vec<LundinResponseJson>> {
    let res = reqwest::get(LUNDIN_BASE_ADDRESS).await?.text().await?;

    Ok(vec![])
}

struct LundinResponseJson {}
