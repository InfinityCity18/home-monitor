use plotters::drawing::IntoDrawingArea;
use plotters_canvas::CanvasBackend;
use yew::prelude::*;

const TEMPERATURE_PLOT_ID = "temperature-plot";

#[function_component]
fn TemperaturePlot() -> Html {

    let backend = CanvasBackend::new(TEMPERATURE_PLOT_ID).expect(format!("Could not get CanvasBackend from {}", TEMPERATURE_PLOT_ID).as_str());
    let root = backend.into_drawing_area();

    html! {
        <div>
            <canvas id={TEMPERATURE_PLOT_ID}/>
        </div>
    }
}
