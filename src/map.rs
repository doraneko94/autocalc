pub mod circle_center;
pub mod dist_sphere;

use yew::prelude::*;

use crate::home::{HomeBase, HomeProps};
use crate::router::Route;

#[function_component(MapHome)]
pub fn map_home(props: &HomeProps) -> Html {
    let lang = props.lang;
    let (home, pages) = (Route::MapHome, vec![Route::MapCircleCenter]);
    html! {
        <>
        <HomeBase {home} {pages} {lang} />
        </>
    }
}