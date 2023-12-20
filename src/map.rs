pub mod circle_center;
pub mod dist_sphere;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::home::{HomeBase, make_card_pages};
use crate::router::parse_query;
use crate::url::{self, Lang};

#[function_component(MapHome)]
pub fn map_home() -> Html {
    let lang = match parse_query(use_location().unwrap().query_str()).1 {
        Some(Lang::Ja) => Lang::Ja, _ => Lang::En
    };
    let pages = make_card_pages(&[url::map_circle_center], lang);
    html! {
        <>
        <HomeBase {pages} {lang} />
        </>
    }
}