use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: u32,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    Sek,
}
