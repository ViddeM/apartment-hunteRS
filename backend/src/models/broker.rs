use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Broker {
    ErikOlsson,
    Lundin,
}
