use yew::prelude::*;
use yew_router::prelude::*;
// use autocalc::header::ja::Header;
use autocalc::router::{MainRoute, switch_main};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}