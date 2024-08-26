use crate::period::set_period;
use crate::period::Period;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct PlotProps {
    pub p: Period,
    pub f: Callback<(Period, String, TableType)>,
    pub id: String,
    pub t: TableType,
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct PlotWindowProps {
    pub f: Callback<(Period, String, TableType)>,
    pub id: String,
    pub t: TableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRequest {
    pub period: Period,
    pub table_type: TableType,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TableType {
    Temperature,
    Humidity,
    Motion,
    Light,
}

impl std::fmt::Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[function_component]
fn Plot(PlotProps { p, f, id, t }: &PlotProps) -> Html {
    let period_clone = p.clone();
    let f = f.clone();
    let id_clone = id.clone();
    let t_clone = t.clone();
    use_effect(move || {
        f.emit((period_clone, id_clone, t_clone));
    });
    html! {
        <div>
            <canvas height=400 width=1400 id={id.clone()}/>
        </div>
    }
}

#[function_component]
pub fn PlotWindow(props: &PlotWindowProps) -> Html {
    let period = use_state(|| Period::Day);

    html! {
        <div>
            <select onchange={set_period(period.clone())}>
                <option selected=true value={Period::Day.to_lowercase_text()}>{format!("Last {}", Period::Day.to_lowercase_text())}</option>
                <option value={Period::Week.to_lowercase_text()}>{format!("Last {}", Period::Week.to_lowercase_text())}</option>
                <option value={Period::Month.to_lowercase_text()}>{format!("Last {}", Period::Month.to_lowercase_text())}</option>
            </select>
            <Plot p={(*period).clone()} f={props.f.clone()} id={props.id.clone()} t={props.t.clone()}/>
        </div>
    }
}
