pub const DOMAIN: &str = "https://autocalc.ushitora.net/";
//pub const DOMAIN: &str = "http://127.0.0.1:8080/";

pub trait Url {
    fn to_url(&self, lang: Lang) -> String;
}

impl Url for &str {
    fn to_url(&self, lang: Lang) -> String {
        let lang_part = match lang {
            Lang::Ja => "lang=ja",
            Lang::En => "lang=en",
        };
        DOMAIN.to_string() + "?p=" + self + "&" + lang_part
    }
}

impl Url for String {
    fn to_url(&self, lang: Lang) -> String {
        self.as_str().to_url(lang)
    }
}

pub fn make_path(path: &str) -> String {
    DOMAIN.to_string() + path
}

#[derive(PartialEq, Clone, Copy)]
pub enum Lang {
    Ja,
    En
}

pub const HOME: &str = "";
pub const UNIT: &str = "unit";
pub const UNIT_LENGTH: &str = "unit/length";
pub const UNIT_MASS: &str = "unit/mass";
pub const MAP: &str = "map";
pub const MAP_CIRCLE_CENTER: &str = "map/circle_center";

use crate::name::*;

pub fn path_to_name(path: &str, lang: Lang) -> &str {
    match path {
        HOME => match lang { Lang::Ja => ja::HOME_NAME, Lang::En => en::HOME_NAME },
        UNIT => match lang { Lang::Ja => ja::UNIT_NAME, Lang::En => en::UNIT_NAME },
        UNIT_LENGTH => match lang { Lang::Ja => ja::UNIT_LENGTH_NAME, Lang::En => en::UNIT_LENGTH_NAME },
        UNIT_MASS => match lang { Lang::Ja => ja::UNIT_MASS_NAME, Lang::En => en::UNIT_MASS_NAME },
        MAP => match lang { Lang::Ja => ja::MAP_NAME, Lang::En => en::MAP_NAME },
        MAP_CIRCLE_CENTER => match lang { Lang::Ja => ja::MAP_CIRCLE_CENTER_NAME, Lang::En => en::MAP_CIRCLE_CENTER_NAME },
        _ => ""
    }
}

pub fn path_and_name(path: &str, lang: Lang) -> (&str, &str) {
    let name = path_to_name(path, lang);
    (path, name)
}