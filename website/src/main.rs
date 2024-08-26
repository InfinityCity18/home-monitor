use crate::plot::TableType;
use plot::PlotWindow;
use yew::prelude::*;

mod consts;
mod humidity;
mod period;
mod plot;
mod temperature;

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    let temp_callback = Callback::from(temperature::draw_plot);
    html! {
    <>
    <PlotWindow f={temp_callback} id={temperature::TEMPERATURE_PLOT_ID} t={TableType::Temperature}/>
    </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
