use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Period {
    Day,
    Week,
    Month,
}

impl Period {
    pub fn to_lowercase_text(&self) -> &str {
        match self {
            Period::Day => "24h",
            Period::Week => "week",
            Period::Month => "month",
        }
    }
    pub fn amount_of_days(&self) -> u64 {
        match self {
            Period::Day => 1,
            Period::Week => 7,
            Period::Month => 30,
        }
    }
    pub fn label_amount(&self) -> usize {
        match self {
            Period::Day => 24,
            Period::Week => 7,
            Period::Month => 30,
        }
    }
}
