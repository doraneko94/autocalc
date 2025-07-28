use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use chrono::Local;
use regex::Regex;

const DOMAIN: &str = "https://autocalc.ushitora.net";

const NUM_LANG: usize = 2;
const LANG: [&str; NUM_LANG] = ["ja", "en"];
const TITLE: [&str; NUM_LANG] = ["AutoCalc", "AutoCalc"];
const SUB_TITLE: [&str; NUM_LANG] = ["高速・高精度のオンライン計算アプリ", "Fast and High-Performance Online Calculation Apps."];
const DESCRIPTION: [&str; NUM_LANG] = [
    "AutoCalcは数字と闘う職人・エンジニアのため、高速かつ高精度の計算をオンラインにて無料で提供します。単位の変換といった日常生活で役立つ計算から、学術論文で活用できる統計関数まで、幅広い計算に対応しています。", 
    "AutoCalc provides fast, accurate calculations online for free for craftsmen and engineers who struggle with numbers. It supports a wide range of computations, from calculations useful in everyday life such as unit conversion to statistical functions that can be used in academic papers."
];
const DATA: [(&str, [&str; NUM_LANG], [&str; NUM_LANG]); 22] = [
    ("privacy", ["プライバシーポリシー・免責事項", "Privacy Policy / Disclaimer"], [
        "AutoCalcのプライバシーポリシー・免責事項です。", 
        "This is Privacy Policy / Disclaimer of AutoCalc."]),

    ("digital", ["デジタル計算", "Digital Computing"], [
        "コンピュータで数字を扱うときのデータ形式を可視化したり、様々な計算を実行したりします。", 
        "These pages show data formats when handling numbers on a computer and perform various calculations."]),

    ("digital/base", ["2進数・8進数・10進数・16進数を相互に変換", "Convert between Binary, Octal, Decimal and Hexadecimal numbers"], [
        "2進数・8進数・10進数・16進数のいずれかで入力された数字を、それ以外の進数に自動変換します。自動のゼロ埋めにも対応しています。", 
        "This automatically converts numbers entered in binary, octal, decimal, or hexadecimal notation to other bases. It also supports automatic zero padding."]),

    ("digital/bit_calc", ["ビット演算", "Bit Calculation"], [
        "NOT, AND, OR, NAND, NOR, XOR, XNORに加え、左右へのシフト演算を実行します。", 
        "Performs NOT, AND, OR, NAND, NOR, XOR, XNOR, plus left and right shift operations."]),

    ("digital/float", ["浮動小数点数の変換 (IEEE 754形式)", "Floating-point conversion (IEEE 754 format)"], [
        "入力された小数をIEEE 754形式の浮動小数点数に変換し、符号・指数部・仮数部の各ビットを表示します。32ビットと64ビットに対応しています。", 
        "Converts the input decimal number to a floating-point number in IEEE 754 format and displays the sign, exponent, and mantissa bits. Supports 32-bit and 64-bit."]),

    ("electronic", ["電気回路・電子回路", "Electrical & Electronic Circuits"], [
        "電気回路や電子回路の設計に役立つ計算を提供します。", 
        "Provides calculations to help you design electrical and electronic circuits."]),

    ("electronic/delta_y", ["デルタ回路・スター回路を相互変換", "Mutual Conversion of Delta and Star Circuits"], [
        "等価なデルタ回路とスター回路に含まれる抵抗値の値をリアルタイムに計算します。", 
        "Calculate in real time the values of the resistances contained in equivalent Delta and Star circuits."]),
    
    ("electronic/resister", ["カラーコードから抵抗値・許容差を計算", "Calculate Resistance and Tolerance from Color Code"], [
        "抵抗器に書かれたカラーコードから、その抵抗値と許容差を計算します。", 
        "Calculate the resistance value and tolerance based on the color code printed on a resistor."]),

    ("map", ["地図・空間情報", "Maps & Spatial Information"], [
        "地図や空間情報を扱うための座標上での計算を提供します。", 
        "Provides coordinate calculations for working with maps and spatial information."]),

    ("map/circle_center", ["３地点から等距離にある地点の緯度経度を計算", "Latitude/Longitude of Points Equidistant from the Three Points"], [
        "３つの地点を緯度経度で設定し、それらから同じ距離にある地点の緯度経度を求めます。", 
        "Set three field points and find the latitude/longitude of the points at the same distance from them."]),

    ("map/dms_float", ["緯度経度の度分秒と浮動小数点数を相互変換", "Convert Latitude and Longitude in Degrees, Minutes, and Deconds to Floating Point Values"], [
        "緯度経度を度・分・秒形式、または小数形式で入力すると、自動的にもう一方の形式に変換します。", 
        "Input latitude and longitude in degrees, minutes, and seconds format or decimal format and it will be automatically converted to the other format."]),

    ("math", ["数学", "Mathematics"], [
        "近似値を計算したり、方程式の解を求めたりします。", 
        "Calculates approximate values and solves equations."]),

    ("math/diffeq_linear2", ["線形微分方程式を解く（小数で出力）", "Solve Linear Differential Equations (Output in Floats)"], [
        "同次線形微分方程式の係数を入力し、その一般解を出力します。出力式の係数は浮動小数点数で与えられます。", 
        "Inputs the coefficients of a homogeneous linear differential equation and outputs its general solution. The coefficients of the output equation are given as floating-point numbers."]),

    ("math/diffeq_linear2_frac", ["線形微分方程式を解く（分数で出力）", "Solve Linear Differential Equations (Output in Fractions)"], [
        "同次線形微分方程式の係数を入力し、その一般解を出力します。出力式では根号・分数を小数に近似せず、そのまま出力します。", 
        "Inputs the coefficients of a homogeneous linear differential equation and outputs its general solution. The output formula does not approximate root signs and fractions to decimals, but outputs them as they are."]),

    ("sport", ["スポーツ", "Sports"], [
        "各種スポーツで使われる指標を自動計算します。", 
        "Automatically calculates indicators used in various sports."]),

    ("sport/golf_sg", ["ゴルフ: 稼いだ打数 (SG)", "Golf: Stroke Gained"], [
        "ストローク前後の球の位置と状況から、ストロークによって稼いだまたは失ったスコアを計算します。", 
        "Calculates the score gained or lost by a stroke, based on the position and situation of the ball before and after the stroke."]),

    ("stat", ["統計計算", "Statistics"], [
        "高度な関数を用いて、統計検定や確率推定のための重要な指標を計算します。", 
        "Use advanced functions to calculate key metrics for statistical tests and probability estimates."]),

    ("stat/error_ellipse", ["2次元正規分布の誤差楕円", "Error Ellipse of 2-D Normal Distribution"], [
        "2次元正規分布の平均と共分散行列から、データのX%が散らばる領域（誤差楕円）の式を計算します。", 
        "From the mean and covariance matrices of a 2-D normal distribution, calculate an expression for the region over which X% of the data is scattered (error ellipse)."]),

    ("stat/roc_auc_ci", ["ROC曲線のAUCがとる信頼区間を計算", "Confidence Interval of ROC-AUC"], [
        "ROC曲線のAUCとサンプル数から、AUCが取りうる値の範囲 (信頼区間) を計算します。", 
        "From the AUC of the ROC curve and the number of samples, the range of values that the AUC can take (confidence interval) is calculated."]),

    ("unit", ["単位変換", "Unit Conversion"], [
        "様々な単位を相互に変換し、それぞれの関係をリアルタイムに表示します。", 
        "These pages converts between various units and displays their relationships in real time."]),

    ("unit/length", ["長さの単位を相互変換", "Interconversion of Length"], [
        "長さの単位を相互に変換します。メートル法・ヤードポンド法・尺貫法のほか、海事系の単位にも対応しています。", 
        "Converts length units into each other. In addition to metric, yard-pound and Japanese (Shaku-Kan) units, it also supports maritime units."]),

    ("unit/mass", ["質量の単位を相互変換", "Interconversion of Mass"], [
        "質量の単位を相互に変換します。メートル法・ヤードポンド法・尺貫法に対応しています。", 
        "Converts mass units into each other. Supports metric, yard-pound and Japanese (Shaku-Kan) units."]),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath= "index.html";
    let html = fs::read_to_string(filepath)?;

    let mut data = Vec::with_capacity((DATA.len() + 1) * NUM_LANG);
    for i in 0..NUM_LANG {
        data.push((
            LANG[i].to_string(),
            if i == 0 { "index".to_string() } else { format!("{}/index", LANG[i]) },
            format!("{} - {}", TITLE[i], SUB_TITLE[i]),
            DESCRIPTION[i].to_string()
        ));
    }

    for &(path, titles, descriptions) in DATA.iter() {
        for i in 0..NUM_LANG {
            data.push((
                LANG[i].to_string(),
                format!("{}{}", if i == 0 { "".to_string() } else { format!("{}/", LANG[i]) }, path),
                format!("{} | {}", titles[i], TITLE[i]),
                descriptions[i].to_string()
            ));
        }
    }

    let xml_path = "../sitemap.xml";
    if let Some(parent) = Path::new(xml_path).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut xml = File::create(xml_path)?;
    writeln!(xml, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    writeln!(xml, "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n")?;

    let today = Local::now().date_naive().format("%Y-%m-%d").to_string();

    let re_title = Regex::new(r"(?s)<title>.*?</title>")?;
    let re_desc = Regex::new(r#"(?i)<meta\s+name=["']description["']\s+content=["'][^"']*["']\s*/?>"#)?;
    let re_og_title = Regex::new(r#"(?i)<meta\s+property=["']og:title["']\s+content=["'][^"']*["']\s*/?>"#)?;
    let re_og_desc = Regex::new(r#"(?i)<meta\s+property=["']og:description["']\s+content=["'][^"']*["']\s*/?>"#)?;
    let re_html_lang = Regex::new(r#"(?i)<html[^>]*lang=["'][^"']*["'][^>]*>"#)?;
    for (i, (lang, path, title, description)) in data.iter().enumerate() {
        println!("{}", path);
        let content = re_title.replace(&html, format!("<title>{}</title>", title));
        let content = re_desc.replace(&content, format!(r#"<meta name="description" content="{}" />"#, description));
        let content = re_og_title.replace(&content, format!(r#"<meta property="og:title" content="{}" />"#, title));
        let content = re_og_desc.replace(&content, format!(r#"<meta property="og:description" content="{}" />"#, description));
        let content = re_html_lang.replace(&content, format!(r#"<html class="h-100" lang="{}">"#, lang));
        let bin_name = path.replace("/", "_");
        let content = content.replace(
            r#"<link data-trunk rel="rust" data-bin="home" />"#, 
            format!(r#"<link data-trunk rel="rust" data-bin="{}" />"#, bin_name).as_str());
        let savepath = format!("../{}.html", bin_name);
        if let Some(parent) = Path::new(&savepath).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(savepath, content.as_bytes())?;

        let url = if i == 0 { "/".to_string() }
        else if i < NUM_LANG { format!("/{}/", LANG[i]) }
        else { format!("/{}/", path) };
        let toml_path = format!("../toml/{}.toml", bin_name);
        if let Some(parent) = Path::new(&toml_path).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&toml_path)?;
        writeln!(file, "[build]")?;
        writeln!(file, "target = \"../{}.html\"", bin_name)?;
        writeln!(file, "public_url = \"{}\"", url)?;

        let _ = Command::new("trunk")
        .arg("build")
        .arg("--release")
        .arg("--dist")
        .arg(format!("../dist{}", url))
        .arg("--config")
        .arg(format!("toml/{}.toml", bin_name))
        .arg("--html-output")
        .arg("index.html")
        .current_dir("..")
        .status()?;

        writeln!(xml, "  <url>")?;
        writeln!(xml, "    <loc>{}{}</loc>", DOMAIN, url)?;
        writeln!(xml, "    <lastmod>{}</lastmod>", today)?;
        writeln!(xml, "    <changefreq>{}</changefreq>", if i < NUM_LANG { "weekly" } else { "monthly" })?;
        writeln!(xml, "    <priority>{}</priority>", if i == 0 { "1.0" } else {
            let mut size = path.split("/").collect::<Vec<&str>>().len();
            if i % NUM_LANG != 0 { size -= 1; }
            if size == 1 { "0.8" } else { "0.6" }
        })?;
        writeln!(xml, "  </url>\n")?;
    }
    writeln!(xml, "</urlset>")?;

    Ok(())
}