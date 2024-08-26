use crate::consts::SERVER_URL;
use crate::period::{set_period, Period};
use chrono::{DateTime, Days, Local};
use full_palette::WHITE;
use gloo_net::http::Request;
use plotters::prelude::*;
use plotters::{chart::ChartBuilder, drawing::IntoDrawingArea};
use plotters_canvas::CanvasBackend;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

const HUMIDITY_PLOT_ID: &str = "humidity-plot";

#[function_component]
pub fn HumidityWindow() -> Html {
    let period = use_state(|| Period::Day);

    html! {
        <div>
            <select onchange={set_period(period.clone())}>
                <option selected=true value={Period::Day.to_lowercase_text()}>{format!("Last {}", Period::Day.to_lowercase_text())}</option>
                <option value={Period::Week.to_lowercase_text()}>{format!("Last {}", Period::Week.to_lowercase_text())}</option>
                <option value={Period::Month.to_lowercase_text()}>{format!("Last {}", Period::Month.to_lowercase_text())}</option>
            </select>
            <HumidityPlot p={(*period).clone()}/>
        </div>
    }
}

#[function_component]
fn HumidityPlot(PlotProps { p }: &PlotProps) -> Html {
    let period_clone = p.clone();
    use_effect(move || {
        draw_plot(period_clone);
    });
    html! {
        <div>
            <canvas height=400 width=1400 id={HUMIDITY_PLOT_ID}/>
        </div>
    }
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct PlotProps {
    p: Period,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRequest {
    pub period: Period,
    pub table_type: String,
}

fn draw_plot(p: Period) {
    wasm_bindgen_futures::spawn_local(async move {
        let backend = CanvasBackend::new(HUMIDITY_PLOT_ID)
            .expect(format!("Could not get CanvasBackend from {}", HUMIDITY_PLOT_ID).as_str());
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
            .x_labels(p.label_amount())
            .x_label_formatter(&p.format_fn())
            .max_light_lines(5)
            .y_desc("Temperature (C°)")
            .draw()
            .expect("Failed to draw on ChartContext");

        let json = ClientRequest {
            period: p,
            table_type: "Temperature".to_string(),
        };

        let response = Request::post(&(SERVER_URL.to_owned() + "/data"))
            .json(&json)
            .expect("Failed to insert json into requestbuilder")
            .send()
            .await
            .expect("Getting data failed");

        let data: Vec<(i64, f64)> = response
            .json()
            .await
            .expect("Deserialization of data failed");

        chart
            .draw_series(LineSeries::new(
                data.iter().map(|(timestamp, v)| {
                    (
                        DateTime::from(DateTime::from_timestamp(*timestamp, 0).unwrap()),
                        *v,
                    )
                }),
                &BLUE,
            ))
            .unwrap();
    })
}
