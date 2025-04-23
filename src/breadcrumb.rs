use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Lang, Route};
use crate::meta::title_dscr;
use crate::utils::to_lang_vec;

#[derive(Properties, PartialEq)]
pub struct BreadCrumbProps {
    pub lang: Lang,
    pub route: Route,
}

#[function_component(BreadCrumb)]
pub fn breadcrumb(props: &BreadCrumbProps) -> Html {
    let lang = props.lang;
    let v_parents = parents(props.route).iter().map(|page| page.to_lang(lang)).collect::<Vec<Route>>();
    let (current_title, _) = title_dscr(props.route.to_lang(lang));
    html!{
        <div class="container">
        <nav style="--bs-breadcrumb-divider: '>';" aria-label="breadcrumb">
            <ol class="breadcrumb">
                {
                    v_parents.iter().map(|route| {
                        let (title, _) = title_dscr(*route);
                        html! {
                            <li class="breadcrumb-item"><Link<Route> to={route.clone()}>{title}</Link<Route>></li>
                        }
                    }).collect::<Html>()
                }
                <li class="breadcrumb-item active" aria-current="page">{current_title}</li>
            </ol>
        </nav>
        </div>
    }
}

fn parents(route: Route) -> Vec<Route> {
    match route {
        Route::Home | Route::HomeEn | Route::NotFound => vec![],
        Route::Privacy | Route::PrivacyEn | Route::DigitalHome | Route::DigitalHomeEn | 
        Route::ElectronicHome | Route::ElectronicHomeEn | Route::MapHome | Route::MapHomeEn |
        Route::MathHome | Route::MathHomeEn | Route::SportHome | Route::SportHomeEn |
        Route::StatHome | Route::StatHomeEn | Route::UnitHome | Route::UnitHomeEn
        => to_lang_vec(&[Route::Home], route.get_lang()),

        Route::DigitalBase | Route::DigitalBaseEn => to_lang_vec(&[Route::Home, Route::DigitalHome], route.get_lang()),
        Route::DigitalBitCalc | Route::DigitalBitCalcEn => to_lang_vec(&[Route::Home, Route::DigitalHome], route.get_lang()),
        Route::DigitalFloat | Route::DigitalFloatEn => to_lang_vec(&[Route::Home, Route::DigitalFloat], route.get_lang()),
        Route::ElectronicDeltaY | Route::ElectronicDeltaYEn => to_lang_vec(&[Route::Home, Route::ElectronicHome], route.get_lang()),
        Route::MapCircleCenter | Route::MapCircleCenterEn => to_lang_vec(&[Route::Home, Route::MapHome], route.get_lang()),
        Route::MathDiffeqLinear2 | Route::MathDiffeqLinear2En => to_lang_vec(&[Route::Home, Route::MathHome], route.get_lang()),
        Route::MathDiffeqLinear2Frac | Route::MathDiffeqLinear2FracEn => to_lang_vec(&[Route::Home, Route::MathHome], route.get_lang()),
        Route::SportGolfSg | Route::SportGolfSgEn => to_lang_vec(&[Route::Home, Route::SportHome], route.get_lang()),
        Route::StatErrorEllipse | Route::StatErrorEllipseEn => to_lang_vec(&[Route::Home, Route::StatHome], route.get_lang()),
        Route::StatRocAucCi | Route::StatRocAucCiEn => to_lang_vec(&[Route::Home, Route::StatHome], route.get_lang()),
        Route::UnitLength | Route::UnitLengthEn => to_lang_vec(&[Route::Home, Route::UnitHome], route.get_lang()),
        Route::UnitMass | Route::UnitMassEn => to_lang_vec(&[Route::Home, Route::UnitHome], route.get_lang()),
    }
}