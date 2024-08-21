use yew::prelude::*;

mod humidity;
mod temperature;

#[function_component(App)]
fn app() -> Html {
    html! {}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
