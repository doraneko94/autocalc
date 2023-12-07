use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <button>{ "Click me!" }</button>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
