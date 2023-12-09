pub mod en;
pub mod ja;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::notfound::NotFound;

pub fn make_path(path: &str) -> String {
    "https://autocalc.ushitora.net/".to_string() + path
    //"http://127.0.0.1:8080/".to_string() + path
}

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! { <><SwitchLang /></> },
        MainRoute::NotFound => html! { <><NotFound /></> }
    }
}

#[function_component(SwitchLang)]
fn switch_lang() -> Html {
    let location = use_location().unwrap();
    let params = parse_query(location.query_str());
    match params {
        (Some(p), Some(lang)) => {
            if lang == "ja".to_string() { html! { <><ja::SwitchPage page={p}/></> } }
            else { html! { <><en::SwitchPage page={p}/></> } }
        },
        (None, Some(lang)) => {
            if lang == "ja".to_string() { html! { <><ja::SwitchPage page={""}/></> } }
            else { html! { <><en::SwitchPage page={""}/></> } }
        },
        (Some(p), None) => { html! { <><en::SwitchPage page={p}/></> } }
        (None, None) => { html! { <><en::SwitchPage page={""}/></> } }
    }
}

pub fn parse_query(query: &str) -> (Option<String>, Option<String>) {
    let query_replace = query.replace("?", "");
    let mut params: (Option<String>, Option<String>) = (None, None);
    let q_list: Vec<&str> = query_replace.split("&").collect();
    for q in q_list.iter() {
        let q_parts:Vec<&str> = q.split("=").collect();
        if q_parts.len() == 2 {
            match q_parts[0] {
                "p" => { params.0 = Some(q_parts[1].to_string()); }
                "lang" => { params.1 = Some(q_parts[1].to_string()); }
                _ => {}
            }
        }
    }
    params
}

pub fn encode_query(params: (Option<String>, Option<String>)) -> String {
    let mut query = "?".to_string();
    match params.0 {
        Some(p) => { query = query + "p=" + &p + "&"; }
        None => {}
    };
    match params.1 {
        Some(lang) => { query = query + "lang=" + &lang; }
        None => {}
    }
    query
}

#[macro_export]
macro_rules! switch_page {
    () => {
        #[derive(Properties, PartialEq)]
        pub struct SwitchPageProps {
            pub page: String,
        }

        #[function_component(SwitchPage)]
        pub fn switch_page(props: &SwitchPageProps) -> Html {
            match props.page.as_str() {
                "" => html! { <><MainHome /></> },
                "unit" => html! { <><UnitHome /></> },
                "unit/length" => html! { <><UnitLength /></> },
                "unit/mass" => html! { <><UnitMass /></> },
                _ => html! { <><NotFound /></> }
            }
        }
    };
}