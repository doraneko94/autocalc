use yew::prelude::*;
use yew_router::prelude::*;

use crate::electronic::ElectronicHome;
use crate::electronic::delta_y::ElectronicDeltaY;
use crate::home::Home;
use crate::map::MapHome;
use crate::map::circle_center::MapCircleCenter;
use crate::math::MathHome;
use crate::math::diffeq_linear2::MathDiffeqLinear2;
use crate::math::diffeq_linear2_frac::MathDiffeqLinear2Frac;
use crate::notfound::NotFound;
use crate::privacy::Privacy;
use crate::sport::SportHome;
use crate::sport::golf_sg::SportGolfSg;
use crate::stat::StatHome;
use crate::stat::error_ellipse::StatErrorEllipse;
use crate::stat::roc_auc_ci::StatRocAucCi;
use crate::unit::UnitHome;
use crate::unit::length::UnitLength;
use crate::unit::mass::UnitMass;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Ja, En
}

#[derive(Clone, Copy, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/privacy")]
    Privacy,
    
    #[at("/electronic/")]
    ElectronicHome,
    #[at("/electronic/delta_y")]
    ElectronicDeltaY,

    #[at("/map/")]
    MapHome,
    #[at("/map/circle_center")]
    MapCircleCenter,
    // #[at("/map/dist_sphere")]
    // MapDistSphere,

    #[at("/math/")]
    MathHome,
    #[at("/math/diffeq_linear2")]
    MathDiffeqLinear2,
    #[at("/math/diffeq_linear2_frac")]
    MathDiffeqLinear2Frac,

    #[at("/sport/")]
    SportHome,
    #[at("/sport/golf_sg")]
    SportGolfSg,

    #[at("/stat/")]
    StatHome,
    #[at("/stat/error_ellipse")]
    StatErrorEllipse,
    #[at("/stat/roc_auc_ci")]
    StatRocAucCi,

    #[at("/unit/")]
    UnitHome,
    #[at("/unit/length")]
    UnitLength,
    #[at("/unit/mass")]
    UnitMass,
    
    #[at("/en/")]
    HomeEn,
    #[at("/en/privacy")]
    PrivacyEn,

    #[at("/en/electronic/")]
    ElectronicHomeEn,
    #[at("/en/electronic/delta_y")]
    ElectronicDeltaYEn,

    #[at("/en/map/")]
    MapHomeEn,
    #[at("/en/map/circle_center")]
    MapCircleCenterEn,
    // #[at("/en/map/dist_sphere")]
    // MapDistSphereEn,

    #[at("/en/math/")]
    MathHomeEn,
    #[at("/en/math/diffeq_linear2")]
    MathDiffeqLinear2En,
    #[at("/en/math/diffeq_linear2_frac")]
    MathDiffeqLinear2FracEn,

    #[at("/en/sport/")]
    SportHomeEn,
    #[at("/en/sport/golf_sg")]
    SportGolfSgEn,

    #[at("/en/stat/")]
    StatHomeEn,
    #[at("/en/stat/error_ellipse")]
    StatErrorEllipseEn,
    #[at("/en/stat/roc_auc_ci")]
    StatRocAucCiEn,

    #[at("/en/unit/")]
    UnitHomeEn,
    #[at("/en/unit/length")]
    UnitLengthEn,
    #[at("/en/unit/mass")]
    UnitMassEn,

    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home | Route::HomeEn => html! { <><Home lang={route.get_lang()} /></> },
        Route::Privacy | Route::PrivacyEn => html! { <><Privacy lang={route.get_lang()} /></> },
        Route::ElectronicHome | Route::ElectronicHomeEn => html! { <><ElectronicHome lang={route.get_lang()} /></> },
        Route::ElectronicDeltaY | Route::ElectronicDeltaYEn => html! { <><ElectronicDeltaY lang={route.get_lang()} /></> },
        Route::MapHome | Route::MapHomeEn => html! { <><MapHome lang={route.get_lang()} /></> },
        Route::MapCircleCenter | Route::MapCircleCenterEn => html! { <><MapCircleCenter lang={route.get_lang()} /></> },
        Route::MathHome | Route::MathHomeEn => html! { <><MathHome lang={route.get_lang()} /></> },
        Route::MathDiffeqLinear2 | Route::MathDiffeqLinear2En => html! { <><MathDiffeqLinear2 lang={route.get_lang()} /></> },
        Route::MathDiffeqLinear2Frac | Route::MathDiffeqLinear2FracEn => html! { <><MathDiffeqLinear2Frac lang={route.get_lang()} /></> },
        Route::SportHome | Route::SportHomeEn => html! { <><SportHome lang={route.get_lang()} /></> },
        Route::SportGolfSg | Route::SportGolfSgEn => html! { <><SportGolfSg lang={route.get_lang()} /></> },
        Route::StatHome | Route::StatHomeEn => html! { <><StatHome lang={route.get_lang()} /></> },
        Route::StatErrorEllipse | Route::StatErrorEllipseEn => html! { <><StatErrorEllipse lang={route.get_lang()} /></> },
        Route::StatRocAucCi | Route::StatRocAucCiEn => html! { <><StatRocAucCi lang={route.get_lang()} /></> },
        Route::UnitHome | Route::UnitHomeEn => html! { <><UnitHome lang={route.get_lang()} /></> },
        Route::UnitLength | Route::UnitLengthEn => html! { <><UnitLength lang={route.get_lang()} /></> },
        Route::UnitMass | Route::UnitMassEn => html! { <><UnitMass lang={route.get_lang()} /></> },

        Route::NotFound => html! { <><NotFound /></> }
    }
}

impl Route {
    pub fn get_lang(&self) -> Lang {
        match self {
            Route::Home | Route::Privacy | Route::ElectronicHome | Route::ElectronicDeltaY
            | Route::MapHome | Route::MapCircleCenter |Route::MathHome | Route::MathDiffeqLinear2 | Route::MathDiffeqLinear2Frac 
            | Route::SportHome | Route::SportGolfSg | Route::StatHome | Route::StatErrorEllipse | Route::StatRocAucCi
            | Route::UnitHome | Route::UnitLength | Route::UnitMass => Lang::Ja,

            _ => Lang::En,
        }
    }

    pub fn to_lang(&self, lang: Lang) -> Route {
        match self {
            Route::Home | Route::HomeEn => match lang { Lang::Ja => Route::Home, Lang::En => Route::HomeEn },
            Route::Privacy | Route::PrivacyEn => match lang { Lang::Ja => Route::Privacy, Lang::En => Route::PrivacyEn },
            Route::ElectronicHome | Route::ElectronicHomeEn => match lang { Lang::Ja => Route::ElectronicHome, Lang::En => Route::ElectronicHomeEn },
            Route::ElectronicDeltaY | Route::ElectronicDeltaYEn => match lang { Lang::Ja => Route::ElectronicDeltaY, Lang::En => Route::ElectronicDeltaYEn },
            Route::MapHome | Route::MapHomeEn => match lang { Lang::Ja => Route::MapHome, Lang::En => Route::MapHomeEn },
            Route::MapCircleCenter | Route::MapCircleCenterEn => match lang { Lang::Ja => Route::MapCircleCenter, Lang::En => Route::MapCircleCenterEn },
            Route::MathHome | Route::MathHomeEn => match lang { Lang::Ja => Route::MathHome, Lang::En => Route::MathHomeEn },
            Route::MathDiffeqLinear2 | Route::MathDiffeqLinear2En => match lang { Lang::Ja => Route::MathDiffeqLinear2, Lang::En => Route::MathDiffeqLinear2En },
            Route::MathDiffeqLinear2Frac | Route::MathDiffeqLinear2FracEn => match lang { Lang::Ja => Route::MathDiffeqLinear2Frac, Lang::En => Route::MathDiffeqLinear2FracEn },
            Route::SportHome | Route::SportHomeEn => match lang { Lang::Ja => Route::SportHome, Lang::En => Route::SportHomeEn },
            Route::SportGolfSg | Route::SportGolfSgEn => match lang { Lang::Ja => Route::SportGolfSg, Lang::En => Route::SportGolfSgEn },
            Route::StatHome | Route::StatHomeEn => match lang { Lang::Ja => Route::StatHome, Lang::En => Route::StatHomeEn },
            Route::StatErrorEllipse | Route::StatErrorEllipseEn => match lang { Lang::Ja => Route::StatErrorEllipse, Lang::En => Route::StatErrorEllipseEn },
            Route::StatRocAucCi | Route::StatRocAucCiEn => match lang { Lang::Ja => Route::StatRocAucCi, Lang::En => Route::StatRocAucCiEn },
            Route::UnitHome | Route::UnitHomeEn => match lang { Lang::Ja => Route::UnitHome, Lang::En => Route::UnitHomeEn },
            Route::UnitLength | Route::UnitLengthEn => match lang { Lang::Ja => Route::UnitLength, Lang::En => Route::UnitLengthEn },
            Route::UnitMass | Route::UnitMassEn => match lang { Lang::Ja => Route::UnitMass, Lang::En => Route::UnitMassEn },
            _ => Route::NotFound
        }
    }
}