pub mod base;
pub mod bit_calc;
pub mod float;

use yew::prelude::*;

use crate::home::{HomeBase, HomeProps};
use crate::router::Route;

#[function_component(DigitalHome)]
pub fn digital_home(props: &HomeProps) -> Html {
    let lang = props.lang;
    let (home, pages) = (Route::DigitalHome, vec![Route::DigitalBase, Route::DigitalFloat, Route::DigitalBitCalc]);
    html! {
        <>
        <HomeBase {home} {pages} {lang} />
        </>
    }
}