use temperature::TemperatureWindow;
use yew::prelude::*;

mod humidity;
mod period;
mod temperature;

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    html! {<TemperatureWindow />}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
