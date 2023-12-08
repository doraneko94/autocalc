mod components;
mod unit;

use yew::prelude::*;
use yew_router::prelude::*;
use components::header::Header;

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

#[function_component(MainHome)]
fn main_home() -> Html {
    html!{
        <>
        <Header />
        <main class="container-fluid mt-2">
            <table class="table">
                <thread>
                    <tr>
                        <th scope="col">{"#"}</th>
                        <th scope="col">{"単位の換算"}</th>
                    </tr>
                </thread>
                <tbody>
                    <tr>
                        <th scope="raw">{1}</th>
                        <td><a href="/unit_length">{"長さの単位を相互変換"}</a></td>
                    </tr>
                    <tr>
                        <th scope="raw">{2}</th>
                        <td><a href="/unit_weight">{"重さの単位を相互変換"}</a></td>
                    </tr>
                </tbody>
            </table>
        </main>
        </>
    }
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <h1>{"Not Found!"}</h1>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/:id")]
    Page { id: String },
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! { <><MainHome /></> },
        MainRoute::Page { id } => 
            match id.as_str() {
                "unit" => html! { <><UnitHome /></> },
                "unit_length" => html! { <><UnitLength /></> },
                "unit_weight" => html! { <><UnitWeight /></> },
                _ => html! { <><NotFound /></> }
            },
        MainRoute::NotFound => html! { <><NotFound /></> }
    }
}

use unit::{
    UnitHome,
    length::UnitLength,
    weight::UnitWeight,
};