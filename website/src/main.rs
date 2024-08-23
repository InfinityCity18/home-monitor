use temperature::TemperatureWindow;
use yew::prelude::*;

mod humidity;
mod period;
mod temperature;

#[function_component(App)]
fn app() -> Html {
    html! {<TemperatureWindow />}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
