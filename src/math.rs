pub mod diffeq_linear2;
pub mod diffeq_linear2_frac;

use yew::prelude::*;

use crate::home::{HomeBase, HomeProps};
use crate::router::Route;

#[function_component(MathHome)]
pub fn math_home(props: &HomeProps) -> Html {
    let lang = props.lang;
    let (home, pages) = (Route::MathHome, vec![Route::MathDiffeqLinear2, Route::MathDiffeqLinear2Frac]);
    html! {
        <>
        <HomeBase {home} {pages} {lang} />
        </>
    }
}