pub mod delta_y;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::home::{HomeBase, make_card_pages};
use crate::router::parse_query;
use crate::url::{self, Lang};

#[function_component(ElectronicHome)]
pub fn electronic_home() -> Html {
    let lang = match parse_query(use_location().unwrap().query_str()).1 {
        Some(Lang::Ja) => Lang::Ja, _ => Lang::En
    };
    let pages = make_card_pages(&[
        url::electronic_delta_y
    ], lang);
    html! {
        <>
        <HomeBase {pages} {lang} />
        </>
    }
}