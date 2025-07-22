//const DOMAIN: &str = "https://localhost:8000";
pub const DOMAIN: &str = "https://autocalc.ushitora.net";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Ja, En
}

#[derive(Clone, Copy, PartialEq)]
pub enum Route {
    Home,
    Privacy,

    DigitalHome,
    DigitalBase,
    DigitalBitCalc,
    DigitalFloat,
    
    ElectronicHome,
    ElectronicDeltaY,
    ElectronicResister,

    MapHome,
    MapCircleCenter,
    // MapDistSphere,
    MapDmsFloat,

    MathHome,
    MathDiffeqLinear2,
    MathDiffeqLinear2Frac,

    SportHome,
    SportGolfSg,

    StatHome,
    StatErrorEllipse,
    StatRocAucCi,

    UnitHome,
    UnitLength,
    UnitMass,
    
    HomeEn,
    PrivacyEn,

    DigitalHomeEn,
    DigitalBaseEn,
    DigitalBitCalcEn,
    DigitalFloatEn,

    ElectronicHomeEn,
    ElectronicDeltaYEn,
    ElectronicResisterEn,

    MapHomeEn,
    MapCircleCenterEn,
    // MapDistSphereEn,
    MapDmsFloatEn,

    MathHomeEn,
    MathDiffeqLinear2En,
    MathDiffeqLinear2FracEn,

    SportHomeEn,
    SportGolfSgEn,

    StatHomeEn,
    StatErrorEllipseEn,
    StatRocAucCiEn,

    UnitHomeEn,
    UnitLengthEn,
    UnitMassEn,

    NotFound
}

impl Route {
    pub fn get_lang(&self) -> Lang {
        match self {
            Route::Home | Route::Privacy | Route::DigitalHome | Route::DigitalBase | Route::DigitalFloat | Route::DigitalBitCalc
            | Route::ElectronicHome | Route::ElectronicDeltaY | Route::MapHome | Route::MapCircleCenter 
            | Route::MathHome | Route::MathDiffeqLinear2 | Route::MathDiffeqLinear2Frac 
            | Route::SportHome | Route::SportGolfSg | Route::StatHome | Route::StatErrorEllipse | Route::StatRocAucCi
            | Route::UnitHome | Route::UnitLength | Route::UnitMass => Lang::Ja,

            _ => Lang::En,
        }
    }

    pub fn to_lang(&self, lang: Lang) -> Route {
        match self {
            Route::Home | Route::HomeEn => match lang { Lang::Ja => Route::Home, Lang::En => Route::HomeEn },
            Route::Privacy | Route::PrivacyEn => match lang { Lang::Ja => Route::Privacy, Lang::En => Route::PrivacyEn },
            Route::DigitalHome | Route::DigitalHomeEn => match lang { Lang::Ja => Route::DigitalHome, Lang::En => Route::DigitalHomeEn },
            Route::DigitalBase | Route::DigitalBaseEn => match lang { Lang::Ja => Route::DigitalBase, Lang::En => Route::DigitalBaseEn },
            Route::DigitalBitCalc | Route::DigitalBitCalcEn => match lang { Lang::Ja => Route::DigitalBitCalc, Lang::En => Route::DigitalBitCalcEn },
            Route::DigitalFloat | Route::DigitalFloatEn => match lang { Lang::Ja => Route::DigitalFloat, Lang::En => Route::DigitalFloatEn },
            Route::ElectronicHome | Route::ElectronicHomeEn => match lang { Lang::Ja => Route::ElectronicHome, Lang::En => Route::ElectronicHomeEn },
            Route::ElectronicDeltaY | Route::ElectronicDeltaYEn => match lang { Lang::Ja => Route::ElectronicDeltaY, Lang::En => Route::ElectronicDeltaYEn },
            Route::ElectronicResister | Route::ElectronicResisterEn => match lang { Lang::Ja => Route::ElectronicResister, Lang::En => Route::ElectronicResisterEn },
            Route::MapHome | Route::MapHomeEn => match lang { Lang::Ja => Route::MapHome, Lang::En => Route::MapHomeEn },
            Route::MapCircleCenter | Route::MapCircleCenterEn => match lang { Lang::Ja => Route::MapCircleCenter, Lang::En => Route::MapCircleCenterEn },
            Route::MapDmsFloat | Route::MapDmsFloatEn => match lang { Lang::Ja => Route::MapDmsFloat, Lang::En => Route::MapDmsFloatEn },
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
            Route::NotFound => Route::NotFound
        }
    }

    pub fn to_url(&self, lang: Lang) -> String {
        match self {
            Route::Home | Route::HomeEn => format!("{}/{}", DOMAIN, add_lang_path(lang)),
            Route::Privacy | Route::PrivacyEn => format!("{}/{}privacy/", DOMAIN, add_lang_path(lang)),
            Route::DigitalHome | Route::DigitalHomeEn => format!("{}/{}digital/", DOMAIN, add_lang_path(lang)),
            Route::DigitalBase | Route::DigitalBaseEn => format!("{}/{}digital/base/", DOMAIN, add_lang_path(lang)),
            Route::DigitalBitCalc | Route::DigitalBitCalcEn => format!("{}/{}digital/bit_calc/", DOMAIN, add_lang_path(lang)),
            Route::DigitalFloat | Route::DigitalFloatEn => format!("{}/{}digital/float/", DOMAIN, add_lang_path(lang)),
            Route::ElectronicHome | Route::ElectronicHomeEn => format!("{}/{}electronic/", DOMAIN, add_lang_path(lang)),
            Route::ElectronicDeltaY | Route::ElectronicDeltaYEn => format!("{}/{}electronic/delta_y/", DOMAIN, add_lang_path(lang)),
            Route::ElectronicResister | Route::ElectronicResisterEn => format!("{}/{}electronic/resister/", DOMAIN, add_lang_path(lang)),
            Route::MapHome | Route::MapHomeEn => format!("{}/{}map/", DOMAIN, add_lang_path(lang)),
            Route::MapCircleCenter | Route::MapCircleCenterEn => format!("{}/{}map/circle_center/", DOMAIN, add_lang_path(lang)),
            Route::MapDmsFloat | Route::MapDmsFloatEn => format!("{}/{}map/dms_float/", DOMAIN, add_lang_path(lang)),
            Route::MathHome | Route::MathHomeEn => format!("{}/{}math/", DOMAIN, add_lang_path(lang)),
            Route::MathDiffeqLinear2 | Route::MathDiffeqLinear2En => format!("{}/{}math/diffeq_linear2/", DOMAIN, add_lang_path(lang)),
            Route::MathDiffeqLinear2Frac | Route::MathDiffeqLinear2FracEn => format!("{}/{}math/diffeq_linear2_frac/", DOMAIN, add_lang_path(lang)),
            Route::SportHome | Route::SportHomeEn => format!("{}/{}sport/", DOMAIN, add_lang_path(lang)),
            Route::SportGolfSg | Route::SportGolfSgEn => format!("{}/{}sport/golf_sg/", DOMAIN, add_lang_path(lang)),
            Route::StatHome | Route::StatHomeEn => format!("{}/{}stat/", DOMAIN, add_lang_path(lang)),
            Route::StatErrorEllipse | Route::StatErrorEllipseEn => format!("{}/{}stat/error_ellipse/", DOMAIN, add_lang_path(lang)),
            Route::StatRocAucCi | Route::StatRocAucCiEn => format!("{}/{}stat/roc_auc_ci/", DOMAIN, add_lang_path(lang)),
            Route::UnitHome | Route::UnitHomeEn => format!("{}/{}unit/", DOMAIN, add_lang_path(lang)),
            Route::UnitLength | Route::UnitLengthEn => format!("{}/{}unit/length/", DOMAIN, add_lang_path(lang)),
            Route::UnitMass | Route::UnitMassEn => format!("{}/{}unit/mass/", DOMAIN, add_lang_path(lang)),
            Route::NotFound => format!("{}/404/", DOMAIN),
        }
    }
}

fn add_lang_path(lang: Lang) -> String {
    match lang { Lang::Ja => "", Lang::En => "en/" }.to_string()
}