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
                        <td><a href="/unit/length">{"長さの単位を相互変換"}</a></td>
                    </tr>
                    <tr>
                        <th scope="raw">{2}</th>
                        <td><a href="/unit/length">{"重さの単位を相互変換"}</a></td>
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
    #[at("/unit")]
    UnitRoot,
    #[at("/unit/*")]
    Unit,
    #[at("/math")]
    Math,
    #[at("/map")]
    Map,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! { <><MainHome /></> },
        MainRoute::UnitRoot | MainRoute::Unit => html! { <Switch<UnitRoute> render={switch_unit} /> },
        MainRoute::Math | MainRoute::Map => html! { <><App /></> },
        MainRoute::NotFound => html! { <><NotFound /></> }
    }
}

use unit::{
    UnitHome,
    length::UnitLength,
    weight::UnitWeight,
};

#[derive(Clone, Routable, PartialEq)]
enum UnitRoute {
    #[at("/unit")]
    Home,
    #[at("/unit/length")]
    Length,
    #[at("/unit/weight")]
    Weight,
    #[not_found]
    #[at("/unit/404")]
    NotFound,
}

fn switch_unit(route: UnitRoute) -> Html {
    match route {
        UnitRoute::Home => html! { <><UnitHome /></> },
        UnitRoute::Length => html! { <><UnitLength /></> },
        UnitRoute::Weight => html! { <><UnitWeight /></> },
        UnitRoute::NotFound => html!{ <Redirect<MainRoute> to={MainRoute::NotFound} /> }
    }
}