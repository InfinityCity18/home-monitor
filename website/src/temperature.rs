use crate::period::{set_period, Period};
use plotters::prelude::*;
use plotters::{chart::ChartBuilder, drawing::IntoDrawingArea};
use plotters_canvas::CanvasBackend;
use yew::prelude::*;

const TEMPERATURE_PLOT_ID: &str = "temperature-plot";

#[function_component]
pub fn TemperatureWindow() -> Html {
    let period = use_state(|| Period::Day);

    html! {
        <div>
            <select>
                <option onclick={set_period(period.clone(), Period::Day)} value={format!("Last {}", Period::Day.to_lowercase_text())}/>
                <option onclick={set_period(period.clone(), Period::Week)} value={format!("Last {}", Period::Week.to_lowercase_text())}/>
                <option onclick={set_period(period.clone(), Period::Month)} value={format!("Last {}", Period::Month.to_lowercase_text())}/>
            </select>
            <TemperaturePlot p={(*period).clone()}/>
        </div>
    }
}

#[function_component]
fn TemperaturePlot(TemperaturePlotProps { p }: &TemperaturePlotProps) -> Html {
    let backend = CanvasBackend::new(TEMPERATURE_PLOT_ID)
        .expect(format!("Could not get CanvasBackend from {}", TEMPERATURE_PLOT_ID).as_str());
    let root = backend.into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(
            format!("Temperature in last {}", p.to_lowercase_text()),
            ("sans-serif", 40),
        )
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Right, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            (Utc.ymd(2010, 1, 1)..Utc.ymd(2018, 12, 1)).monthly(),
            14.0..104.0,
        )
        .unwrap();

    html! {
        <div>
            <canvas id={TEMPERATURE_PLOT_ID}/>
        </div>
    }
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct TemperaturePlotProps {
    p: Period,
}
