use yew::{Callback, MouseEvent, UseStateHandle};

#[derive(Clone, Debug, PartialEq)]
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
}

pub fn set_period(handle: UseStateHandle<Period>, p: Period) -> Callback<MouseEvent> {
    Callback::from(move |_| handle.set(p.clone()))
}
