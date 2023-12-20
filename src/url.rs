pub const DOMAIN: &str = "https://autocalc.ushitora.net/";
//pub const DOMAIN: &str = "http://127.0.0.1:8080/";

#[derive(PartialEq, Clone, Copy)]
pub enum DataMode {
    Route,
    Url(Lang),
    Name(Lang),
    Dscr(Lang),
}

macro_rules! data {
    (
        $f: ident, $route: expr,
        $name_ja: expr, $name_en: expr,
        $dscr_ja: expr,
        $dscr_en: expr
    ) => {
        pub fn $f(mode: DataMode) -> String {
            match mode {
                DataMode::Route => $route.to_string(),
                DataMode::Url(lang) => match lang {
                    Lang::Ja => DOMAIN.to_string() + "?p=" + $route + "&lang=ja",
                    Lang::En => DOMAIN.to_string() + "?p=" + $route + "&lang=en",
                },
                DataMode::Name(lang) => match lang {
                    Lang::Ja => $name_ja.to_string(),
                    Lang::En => $name_en.to_string(),
                },
                DataMode::Dscr(lang) => match lang {
                    Lang::Ja => $dscr_ja.to_string(),
                    Lang::En => $dscr_en.to_string(),
                }
            }
        }
    };
}

data!(home, "", "TOP", "TOP", "", "");
data!(map, "map", "地図", "Map", "", "");
data!(
    map_circle_center, "map/circle_center",
    "３地点から等距離にある地点の緯度経度を計算", "Latitude/Longitude of Points Equidistant from the Three Points",
    "３つの地点を設定し、それらから同じ距離にある地点の緯度経度を求めます。",
    "Set three field points and find the latitude/longitude of the points at the same distance from them."
);
data!(stat, "stat", "統計", "Statistics", "", "");
data!(
    stat_roc_auc_ci, "stat/roc_auc_ci",
    "ROC曲線のAUCがとる信頼区間を計算", "Confidence interval of ROC-AUC",
    "ROC曲線のAUCとサンプル数から、AUCが取りうる値の範囲（信頼区間）を計算します。",
    "From the AUC of the ROC curve and the number of samples, the range of values that the AUC can take (confidence interval) is calculated."
);
data!(unit, "unit", "単位の換算", "Conversion of Units", "", "");
data!(
    unit_length, "unit/length",
    "長さの単位を相互変換", "Interconversion of Length",
    "長さの単位を相互に変換します。メートル法・ヤードポンド法・尺貫法のほか、海事系の単位にも対応しています。",
    "Converts length units into each other. In addition to metric, yard-pound and Japanese (Shaku-Kan) units, it also supports maritime units."
);
data!(
    unit_mass, "unit/mass",
    "質量の単位を相互変換", "Interconversion of Mass",
    "質量の単位を相互に変換します。メートル法・ヤードポンド法・尺貫法に対応しています。",
    "Converts mass units into each other. Supports metric, yard-pound and Japanese (Shaku-Kan) units."
);
data!(
    privacy, "privacy",
    "プライバシーポリシー・免責事項", "Privacy Policy / Disclaimer",
    "", ""
);

#[derive(PartialEq, Clone, Copy)]
pub enum Lang {
    Ja,
    En
}

macro_rules! path2name {
    ($f: ident, $path: ident, $lang: ident) => {
        if $f(DataMode::Route) == $path { return $f(DataMode::Name($lang)) }
    };
}

pub fn path_to_name(path: &str, lang: Lang) -> String {
    let path = path.to_string();
    path2name!(home, path, lang);
    path2name!(map, path, lang);
    path2name!(map_circle_center, path, lang);
    path2name!(stat, path, lang);
    path2name!(stat_roc_auc_ci, path, lang);
    path2name!(unit, path, lang);
    path2name!(unit_length, path, lang);
    path2name!(unit_mass, path, lang);
    path2name!(privacy, path, lang);

    "".to_string()
}

pub fn url_and_name<F: Fn(DataMode)->String>(data: F, lang: Lang) -> (String, String) {
    (data(DataMode::Url(lang)), data(DataMode::Name(lang)))
}

pub fn query_to_url(query: &str) -> String {
    DOMAIN.to_string() + query
}

pub fn path_to_url(path: &str, lang: Lang) -> String {
    match lang {
        Lang::Ja => DOMAIN.to_string() + "?p=" + path + "&lang=ja",
        Lang::En => DOMAIN.to_string() + "?p=" + path + "&lang=en",
    }
}

#[macro_export]
macro_rules! set_lang {
    ($f: ident, $ja: expr, $en: expr) => {
        fn $f(lang: Lang) -> String {
            match lang {
                Lang::Ja => $ja.to_string(),
                Lang::En => $en.to_string(),
            }
        }
    };
}