use crate::period::{set_period, Period};
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
                <option onclick={set_period(period.clone(), Period::Day)} value="Last 24h"/>
                <option onclick={set_period(period.clone(), Period::Week)} value="Last week"/>
                <option onclick={set_period(period.clone(), Period::Month)} value="Last month"/>
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

    let mut chart = ChartBuilder::on(&root).margin(10).caption(
        format!("Temperature in last {}", p.to_lowercase_text()),
        style,
    );

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
