use crate::period::{set_period, Period};
use chrono::{Days, Local, Timelike};
use full_palette::WHITE;
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
            <select onchange={set_period(period.clone())}>
                <option selected=true value={Period::Day.to_lowercase_text()}>{format!("Last {}", Period::Day.to_lowercase_text())}</option>
                <option value={Period::Week.to_lowercase_text()}>{format!("Last {}", Period::Week.to_lowercase_text())}</option>
                <option value={Period::Month.to_lowercase_text()}>{format!("Last {}", Period::Month.to_lowercase_text())}</option>
            </select>
            <TemperaturePlot p={(*period).clone()}/>
        </div>
    }
}

#[function_component]
fn TemperaturePlot(TemperaturePlotProps { p }: &TemperaturePlotProps) -> Html {
    let period_clone = p.clone();
    use_effect(move || {
        draw_plot(period_clone);
    });
    html! {
        <div>
            <canvas height=400 width=1500 id={TEMPERATURE_PLOT_ID}/>
        </div>
    }
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct TemperaturePlotProps {
    p: Period,
}

fn draw_plot(p: Period) {
    let backend = CanvasBackend::new(TEMPERATURE_PLOT_ID)
        .expect(format!("Could not get CanvasBackend from {}", TEMPERATURE_PLOT_ID).as_str());
    let root = backend.into_drawing_area();

    let end = Local::now();
    let start = end - Days::new(p.amount_of_days());

    root.fill(&WHITE).expect("Filling failed");

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(
            format!("Temperature in last {}", p.to_lowercase_text()),
            ("sans-serif", 40),
        )
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Right, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(start..end, 0.0..45.0)
        .expect("Failed to build chart");

    chart
        .configure_mesh()
        .disable_x_mesh()
        .x_labels(24)
        .x_label_formatter(&p.format_fn())
        .max_light_lines(4)
        .y_desc("Temperature (C°)")
        .draw()
        .expect("Failed to draw on ChartContext");
}
