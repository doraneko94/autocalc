pub mod golf_sg;

use yew::prelude::*;

use crate::home::{HomeBase, HomeProps};
use crate::router::{Lang, Route};

#[function_component(SportHome)]
pub fn sport_home(props: &HomeProps) -> Html {
    let lang = props.lang;
    let (home, pages) = match lang {
        Lang::Ja => (Route::SportHome, vec![Route::SportGolfSg]),
        Lang::En => (Route::SportHomeEn, vec![Route::SportGolfSgEn])
    };
    html! {
        <>
        <HomeBase {home} {pages} {lang} />
        </>
    }
}