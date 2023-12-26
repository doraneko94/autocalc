pub mod diffeq_linear2;
pub mod diffeq_linear2_frac;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::home::{HomeBase, make_card_pages};
use crate::router::parse_query;
use crate::url::{self, Lang};

#[function_component(MathHome)]
pub fn math_home() -> Html {
    let lang = match parse_query(use_location().unwrap().query_str()).1 {
        Some(Lang::Ja) => Lang::Ja, _ => Lang::En
    };
    let pages = make_card_pages(&[
        url::math_diffeq_linear2,
        url::math_diffeq_linear2_frac
    ], lang);
    html! {
        <>
        <HomeBase {pages} {lang} />
        </>
    }
}