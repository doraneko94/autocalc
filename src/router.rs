use yew::prelude::*;
use yew_router::prelude::*;

use crate::home::MainHome;
use crate::map::MapHome;
use crate::map::circle_center::MapCircleCenter;
use crate::notfound::NotFound;
use crate::privacy::Privacy;
use crate::stat::StatHome;
use crate::stat::roc_auc_ci::StatRocAucCi;
use crate::unit::UnitHome;
use crate::unit::length::UnitLength;
use crate::unit::mass::UnitMass;
use crate::url::*;

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
        MainRoute::Home => html! { <><SwitchPage /></> },
        MainRoute::NotFound => html! { <><NotFound /></> }
    }
}

pub fn parse_query(query: &str) -> (Option<String>, Option<Lang>) {
    let query_replace = query.replace("?", "");
    let mut params: (Option<String>, Option<Lang>) = (None, None);
    let q_list: Vec<&str> = query_replace.split("&").collect();
    for q in q_list.iter() {
        let q_parts:Vec<&str> = q.split("=").collect();
        if q_parts.len() == 2 {
            match q_parts[0] {
                "p" => { params.0 = Some(q_parts[1].to_string()); }
                "lang" => {
                    params.1 = match q_parts[1] {
                        "ja" => Some(Lang::Ja),
                        _ => Some(Lang::En)
                    }
                }
                _ => {}
            }
        }
    }
    params
}

pub fn encode_query(params: (Option<String>, Option<Lang>)) -> String {
    let mut query = "?".to_string();
    match params.0 {
        Some(p) => { query = query + "p=" + &p + "&"; }
        None => {}
    };
    match params.1 {
        Some(lang) => {
            query = query + "lang=" + match lang {
                Lang::Ja => "ja",
                Lang::En => "en",
            };
        }
        None => {}
    }
    query
}

macro_rules! switch {
    ($f: ident, $path: ident, $page: ident) => {
        if $f(DataMode::Route) == $path { return html! { <><$page /></> } }
    };
}

#[function_component(SwitchPage)]
pub fn switch_page() -> Html {
    let params = parse_query(use_location().unwrap().query_str());
    let path = match params {
        (Some(p), _) => p,
        (None, _) => "".to_string(),
    };

    switch!(home, path, MainHome);
    switch!(unit, path, UnitHome);
    switch!(unit_length, path, UnitLength);
    switch!(unit_mass, path, UnitMass);
    switch!(map, path, MapHome);
    switch!(map_circle_center, path, MapCircleCenter);
    switch!(stat, path, StatHome);
    switch!(stat_roc_auc_ci, path, StatRocAucCi);
    switch!(privacy, path, Privacy);
    html! { <><NotFound /></> }
}