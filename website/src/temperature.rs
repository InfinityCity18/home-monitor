use crate::consts::SERVER_URL;
use crate::period::{set_period, Period};
use crate::plot::{ClientRequest, TableType};
use chrono::{DateTime, Days, Local};
use full_palette::WHITE;
use gloo_net::http::Request;
use plotters::prelude::*;
use plotters::{chart::ChartBuilder, drawing::IntoDrawingArea};
use plotters_canvas::CanvasBackend;

pub const TEMPERATURE_PLOT_ID: &str = "temperature-plot";

pub fn draw_plot((p, id, t): (Period, String, TableType)) {
    wasm_bindgen_futures::spawn_local(async move {
        let backend = CanvasBackend::new(&id)
            .expect(format!("Could not get CanvasBackend from {}", &id).as_str());
        let root = backend.into_drawing_area();

        let end = Local::now();
        let start = end - Days::new(p.amount_of_days());

        root.fill(&WHITE).expect("Filling failed");

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(
                format!("{} in last {}", t.to_string(), p.to_lowercase_text()),
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
            table_type: t,
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
