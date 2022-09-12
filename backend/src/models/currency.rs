#[derive(Debug, Clone)]
pub struct Money {
    pub amount: u32,
    pub currency: Currency,
}

#[derive(Debug, Clone)]
pub enum Currency {
    Sek,
}
