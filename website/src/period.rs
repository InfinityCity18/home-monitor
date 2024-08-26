use chrono::{DateTime, Datelike, Local, Month, Timelike};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;

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
    pub fn format_fn(&self) -> impl Fn(&DateTime<Local>) -> String + '_ {
        move |x| match self {
            Period::Day => format!("{}:{:02}", x.hour(), x.minute()),
            Period::Week => {
                let month = Month::try_from(x.month() as u8)
                    .expect("Conversion from u8 to Month failed")
                    .name();
                format!("{} {}", &month[0..3], x.day())
            }
            Period::Month => {
                let month = Month::try_from(x.month() as u8)
                    .expect("Conversion from u8 to Month failed")
                    .name();
                format!("{} {}", &month[0..3], x.day())
            }
        }
    }
}

pub fn set_period(handle: UseStateHandle<Period>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

        if let Some(input) = input {
            let i = input.value();
            let p = match i {
                s if s == Period::Day.to_lowercase_text() => Period::Day,
                s if s == Period::Week.to_lowercase_text() => Period::Week,
                s if s == Period::Month.to_lowercase_text() => Period::Month,
                _ => unreachable!(),
            };
            handle.set(p);
        }
    })
}
