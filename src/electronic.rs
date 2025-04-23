pub mod delta_y;

use yew::prelude::*;

use crate::home::{HomeBase, HomeProps};
use crate::router::Route;

#[function_component(ElectronicHome)]
pub fn electronic_home(props: &HomeProps) -> Html {
    let lang = props.lang;
    let (home, pages) = (Route::ElectronicHome, vec![Route::ElectronicDeltaY]);
    html! {
        <>
        <HomeBase {home} {pages} {lang} />
        </>
    }
}