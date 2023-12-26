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
data!(electronic, "electronic", "電子回路", "Electronic Circuit", "", "");
data!(electronic_delta_y, "electronic/delta_y", "デルタ回路・スター回路を相互変換", "Mutual Conversion of Delta and Star Circuits.", 
    "等価なデルタ回路とスター回路に含まれる抵抗値の値をリアルタイムに計算します。", 
    "Calculate in real time the values of the resistances contained in equivalent Delta and Star circuits."
);
data!(map, "map", "地図", "Map", "", "");
data!(
    map_circle_center, "map/circle_center",
    "３地点から等距離にある地点の緯度経度を計算", "Latitude/Longitude of Points Equidistant from the Three Points",
    "３つの地点を設定し、それらから同じ距離にある地点の緯度経度を求めます。",
    "Set three field points and find the latitude/longitude of the points at the same distance from them."
);
data!(math, "math", "数学", "Mathematics", "", "");
data!(
    math_diffeq_linear2, "math/diffeq_linear2",
    "線形微分方程式を解く（小数で出力）", "Solve linear differential equations (Output in decimals)", 
    "同次線形微分方程式の係数を入力し、その一般解を出力します。出力式の係数は浮動小数点数で与えられます。", 
    "Inputs the coefficients of a homogeneous linear differential equation and outputs its general solution. The coefficients of the output equation are given as floating-point numbers."
);
data!(
    math_diffeq_linear2_frac, "math/diffeq_linear2_frac",
    "線形微分方程式を解く（分数で出力）", "Solve linear differential equations (Output in fractions)", 
    "同次線形微分方程式の係数を入力し、その一般解を出力します。出力式では根号・分数を小数に近似せず、そのまま出力します。", 
    "Inputs the coefficients of a homogeneous linear differential equation and outputs its general solution. The output formula does not approximate root signs and fractions to decimals, but outputs them as they are."
);
data!(stat, "stat", "統計", "Statistics", "", "");
data!(
    stat_error_ellipse, "stat/error_ellipse",
    "2次元正規分布の誤差楕円", "Error Ellipse of 2-D Normal Distribution",
    "2次元正規分布の平均と共分散行列から、データのX%が散らばる領域（誤差楕円）の式を計算します。",
    "From the mean and covariance matrices of a 2-D normal distribution, calculate an expression for the region over which X% of the data is scattered (error ellipse)."
);
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
    path2name!(electronic, path, lang);
    path2name!(electronic_delta_y, path, lang);
    path2name!(map, path, lang);
    path2name!(map_circle_center, path, lang);
    path2name!(math, path, lang);
    path2name!(math_diffeq_linear2, path, lang);
    path2name!(math_diffeq_linear2_frac, path, lang);
    path2name!(stat, path, lang);
    path2name!(stat_error_ellipse, path, lang);
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