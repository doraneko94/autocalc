use yew::prelude::*;

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
                            <li class="breadcrumb-item"><a href={route.to_url(lang)}>{title}</a></li>
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

        Route::DigitalBase | Route::DigitalBaseEn => vec![Route::Home, Route::DigitalHome],
        Route::DigitalBitCalc | Route::DigitalBitCalcEn => vec![Route::Home, Route::DigitalHome],
        Route::DigitalFloat | Route::DigitalFloatEn => vec![Route::Home, Route::DigitalHome],
        Route::ElectronicDeltaY | Route::ElectronicDeltaYEn => vec![Route::Home, Route::ElectronicHome],
        Route::ElectronicResister | Route::ElectronicResisterEn => vec![Route::Home, Route::ElectronicHome],
        Route::MapCircleCenter | Route::MapCircleCenterEn => vec![Route::Home, Route::MapHome],
        Route::MapDmsFloat | Route::MapDmsFloatEn => vec![Route::Home, Route::MapHome],
        Route::MathDiffeqLinear2 | Route::MathDiffeqLinear2En => vec![Route::Home, Route::MathHome],
        Route::MathDiffeqLinear2Frac | Route::MathDiffeqLinear2FracEn => vec![Route::Home, Route::MathHome],
        Route::SportGolfSg | Route::SportGolfSgEn => vec![Route::Home, Route::SportHome],
        Route::StatErrorEllipse | Route::StatErrorEllipseEn => vec![Route::Home, Route::StatHome],
        Route::StatRocAucCi | Route::StatRocAucCiEn => vec![Route::Home, Route::StatHome],
        Route::UnitLength | Route::UnitLengthEn => vec![Route::Home, Route::UnitHome],
        Route::UnitMass | Route::UnitMassEn => vec![Route::Home, Route::UnitHome],
    }
}