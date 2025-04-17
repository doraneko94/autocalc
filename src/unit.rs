pub mod length;
pub mod mass;

use yew::prelude::*;

use crate::home::{HomeBase, HomeProps};
use crate::router::{Lang, Route};

#[function_component(UnitHome)]
pub fn unit_home(props: &HomeProps) -> Html {
    let lang = props.lang;
    let (home, pages) = match lang {
        Lang::Ja => (Route::UnitHome, vec![Route::UnitLength, Route::UnitMass]),
        Lang::En => (Route::UnitHomeEn, vec![Route::UnitLengthEn, Route::UnitMassEn])
    };
    html! {
        <>
        <HomeBase {home} {pages} {lang} />
        </>
    }
}