use crate::consts::SERVER_URL;
use crate::period::Period;
use crate::plot::{ClientRequest, TableType, BG_COLOR};
use chrono::{DateTime, Days, Local};
use full_palette::WHITE;
use gloo_net::http::Request;
use plotters::prelude::*;
use plotters::{chart::ChartBuilder, drawing::IntoDrawingArea};
use plotters_canvas::CanvasBackend;

pub const MOTION_PLOT_ID: &str = "motion-plot";

pub fn draw_plot((p, id, t): (Period, String, TableType)) {
    wasm_bindgen_futures::spawn_local(async move {
        let backend = CanvasBackend::new(&id)
            .expect(format!("Could not get CanvasBackend from {}", &id).as_str());
        let root = backend.into_drawing_area();

        let style = ("sans-serif", 40, &WHITE);
        let shape_style = ShapeStyle {
            color: WHITE.into(),
            filled: true,
            stroke_width: 2,
        };

        let end = Local::now();
        let start = end - Days::new(p.amount_of_days());

        root.fill(&BG_COLOR).expect("Filling failed");

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(
                format!("{} in last {}", t.to_string(), p.to_lowercase_text()),
                style,
            )
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Right, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(start..end, 0.0..1.0)
            .expect("Failed to build chart");

        chart
            .configure_mesh()
            .disable_x_mesh()
            .x_labels(p.label_amount())
            .x_label_formatter(&p.format_fn())
            .axis_style(shape_style)
            .label_style(&WHITE)
            .max_light_lines(5)
            .y_desc("Motion (true or false)")
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

        let data: Vec<(i64, i64)> = response
            .json()
            .await
            .expect("Deserialization of data failed");

        chart
            .draw_series(AreaSeries::new(
                data.iter().map(|(timestamp, v)| {
                    (
                        DateTime::from(DateTime::from_timestamp(*timestamp, 0).unwrap()),
                        *v as f64,
                    )
                }),
                0.0,
                &WHITE.mix(0.3),
            ))
            .unwrap();
    })
}
