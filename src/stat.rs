pub mod roc_auc_ci;
pub mod error_ellipse;

use yew::prelude::*;

use crate::home::{HomeBase, HomeProps};
use crate::router::{Lang, Route};

#[function_component(StatHome)]
pub fn stat_home(props: &HomeProps) -> Html {
    let lang = props.lang;
    let (home, pages) = match lang {
        Lang::Ja => (Route::StatHome, vec![Route::StatRocAucCi, Route::StatErrorEllipse]),
        Lang::En => (Route::StatHomeEn, vec![Route::StatRocAucCiEn, Route::StatErrorEllipseEn])
    };
    html! {
        <>
        <HomeBase {home} {pages} {lang} />
        </>
    }
}