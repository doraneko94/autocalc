pub mod delta_y;

use yew::prelude::*;

use crate::home::HomeBase;
use crate::router::{Lang, Route};

#[derive(Properties, PartialEq)]
pub struct ElectronicHomeProps {
    pub lang: Lang,
}

#[function_component(ElectronicHome)]
pub fn electronic_home(props: &ElectronicHomeProps) -> Html {
    let lang = props.lang;
    let (home, pages) = (Route::ElectronicHome, vec![Route::ElectronicDeltaY]);
    html! {
        <>
        <HomeBase {home} {pages} {lang} />
        </>
    }
}