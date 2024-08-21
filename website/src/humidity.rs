use yew::prelude::*;

#[function_component]
fn HumidityPlot() -> Html {
    html! {
        <div>
            <canvas id={"humidity-plot"}/>
        </div>
    }
}
