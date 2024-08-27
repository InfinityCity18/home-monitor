use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Period {
    Day,
    Week,
    Month,
}

impl Period {
    pub fn amount_of_days(&self) -> u64 {
        match self {
            Period::Day => 1,
            Period::Week => 7,
            Period::Month => 30,
        }
    }
}
