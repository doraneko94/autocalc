use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Lang, Route};
use crate::meta::title_dscr;

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
        Route::Home => vec![],
        Route::Privacy => vec![Route::Home],
        Route::ElectronicHome => vec![Route::Home],
        Route::ElectronicDeltaY => vec![Route::Home, Route::ElectronicHome],
        Route::MapHome => vec![Route::Home],
        Route::MapCircleCenter => vec![Route::Home, Route::MapHome],
        Route::MathHome => vec![Route::Home],
        Route::MathDiffeqLinear2 => vec![Route::Home, Route::MathHome],
        Route::MathDiffeqLinear2Frac => vec![Route::Home, Route::MathHome],
        Route::SportHome => vec![Route::Home],
        Route::SportGolfSg => vec![Route::Home, Route::SportHome],
        Route::StatHome => vec![Route::Home],
        Route::StatErrorEllipse => vec![Route::Home, Route::StatHome],
        Route::StatRocAucCi => vec![Route::Home, Route::StatHome],
        Route::UnitHome => vec![Route::Home],
        Route::UnitLength => vec![Route::Home, Route::UnitHome],
        Route::UnitMass => vec![Route::Home, Route::UnitHome],

        Route::HomeEn => vec![],
        Route::PrivacyEn => vec![Route::HomeEn],
        Route::ElectronicHomeEn => vec![Route::HomeEn],
        Route::ElectronicDeltaYEn => vec![Route::HomeEn, Route::ElectronicHomeEn],
        Route::MapHomeEn => vec![Route::HomeEn],
        Route::MapCircleCenterEn => vec![Route::HomeEn, Route::MapHomeEn],
        Route::MathHomeEn => vec![Route::HomeEn],
        Route::MathDiffeqLinear2En => vec![Route::HomeEn, Route::MathHomeEn],
        Route::MathDiffeqLinear2FracEn => vec![Route::HomeEn, Route::MathHomeEn],
        Route::SportHomeEn => vec![Route::HomeEn],
        Route::SportGolfSgEn => vec![Route::HomeEn, Route::SportHomeEn],
        Route::StatHomeEn => vec![Route::HomeEn],
        Route::StatErrorEllipseEn => vec![Route::HomeEn, Route::StatHomeEn],
        Route::StatRocAucCiEn => vec![Route::HomeEn, Route::StatHomeEn],
        Route::UnitHomeEn => vec![Route::HomeEn],
        Route::UnitLengthEn => vec![Route::HomeEn, Route::UnitHomeEn],
        Route::UnitMassEn => vec![Route::HomeEn, Route::UnitHomeEn],

        Route::NotFound => vec![]
    }
}