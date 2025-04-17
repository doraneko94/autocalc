use yew::prelude::*;
use yew_router::prelude::*;
use autocalc::router::{Route, switch};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}