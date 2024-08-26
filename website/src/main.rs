use crate::plot::TableType;
use plot::PlotWindow;
use yew::prelude::*;

mod consts;
mod humidity;
mod light;
mod motion;
mod period;
mod plot;
mod temperature;

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    let temp_callback = Callback::from(temperature::draw_plot);
    let humd_callback = Callback::from(humidity::draw_plot);
    let motion_callback = Callback::from(motion::draw_plot);
    let light_callback = Callback::from(light::draw_plot);
    html! {
    <>
    <PlotWindow f={temp_callback} id={temperature::TEMPERATURE_PLOT_ID} t={TableType::Temperature}/>
    <PlotWindow f={humd_callback} id={humidity::HUMIDITY_PLOT_ID} t={TableType::Humidity}/>
    <PlotWindow f={motion_callback} id={motion::MOTION_PLOT_ID} t={TableType::Motion}/>
    <PlotWindow f={light_callback} id={light::LIGHT_PLOT_ID} t={TableType::Light}/>
    </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
