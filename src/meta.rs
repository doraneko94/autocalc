use crate::router::Route;

pub fn title_dscr(route: Route) -> (String, String) {
    let (title, dscr) = match route {
        Route::Home => (
            "TOP", 
            ""
        ),
        Route::Privacy => (
            "プライバシーポリシー・免責事項",
            ""
        ),
        Route::ElectronicHome => (
            "電子回路",
            ""
        ),
        Route::ElectronicDeltaY => (
            "デルタ回路・スター回路を相互変換",
            "等価なデルタ回路とスター回路に含まれる抵抗値の値をリアルタイムに計算します。", 
        ),
        Route::MapHome => (
            "地図",
            ""
        ),
        Route::MapCircleCenter => (
            "３地点から等距離にある地点の緯度経度を計算",
            "３つの地点を設定し、それらから同じ距離にある地点の緯度経度を求めます。"
        ),
        Route::MathHome => (
            "数学",
            ""
        ),
        Route::MathDiffeqLinear2 => (
            "線形微分方程式を解く（小数で出力）",
            "同次線形微分方程式の係数を入力し、その一般解を出力します。出力式の係数は浮動小数点数で与えられます。"
        ),
        Route::MathDiffeqLinear2Frac => (
            "線形微分方程式を解く（分数で出力）",
            "同次線形微分方程式の係数を入力し、その一般解を出力します。出力式では根号・分数を小数に近似せず、そのまま出力します。", 
        ),
        Route::SportHome => (
            "スポーツ", 
            ""
        ),
        Route::SportGolfSg => (
            "ゴルフ：稼いだ打数（SG）",
            "ストローク前後の球の位置と状況から、ストロークによって稼いだまたは失ったスコアを計算します。"
        ),
        Route::StatHome => (
            "統計",
            ""
        ),
        Route::StatErrorEllipse => (
            "2次元正規分布の誤差楕円",
            "2次元正規分布の平均と共分散行列から、データのX%が散らばる領域（誤差楕円）の式を計算します。"
        ),
        Route::StatRocAucCi => (
            "ROC曲線のAUCがとる信頼区間を計算",
            "ROC曲線のAUCとサンプル数から、AUCが取りうる値の範囲（信頼区間）を計算します。"
        ),
        Route::UnitHome => (
            "単位の換算", 
            ""
        ),
        Route::UnitLength => (
            "長さの単位を相互変換", 
            "長さの単位を相互に変換します。メートル法・ヤードポンド法・尺貫法のほか、海事系の単位にも対応しています。"
        ),
        Route::UnitMass => (
            "質量の単位を相互変換",
            "質量の単位を相互に変換します。メートル法・ヤードポンド法・尺貫法に対応しています。"
        ),

        Route::HomeEn => (
            "TOP",
            ""
        ),
        Route::ElectronicHomeEn => (
            "Electronic Circuit",
            ""
        ),
        Route::ElectronicDeltaYEn => (
            "Mutual Conversion of Delta and Star Circuits",
            "Calculate in real time the values of the resistances contained in equivalent Delta and Star circuits."
        ),
        Route::MapHomeEn => (
            "Map",
            ""
        ),
        Route::MapCircleCenterEn => (
            "Latitude/Longitude of Points Equidistant from the Three Points",
            "Set three field points and find the latitude/longitude of the points at the same distance from them."
        ),
        Route::MathHomeEn => (
            "Mathematics",
            ""
        ),
        Route::MathDiffeqLinear2En => (
            "Solve linear differential equations (Output in decimals)",
            "Inputs the coefficients of a homogeneous linear differential equation and outputs its general solution. The coefficients of the output equation are given as floating-point numbers."
        ),
        Route::MathDiffeqLinear2FracEn => (
            "Solve linear differential equations (Output in fractions)",
            "Inputs the coefficients of a homogeneous linear differential equation and outputs its general solution. The output formula does not approximate root signs and fractions to decimals, but outputs them as they are."
        ),
        Route::SportHomeEn => (
            "sports", 
            ""
        ),
        Route::SportGolfSgEn => (
            "Golf: Stroke Gained",
            "Calculates the score gained or lost by a stroke, based on the position and situation of the ball before and after the stroke."
        ),
        Route::StatHomeEn => (
            "Statistics",
            ""
        ),
        Route::StatErrorEllipseEn => (
            "Error Ellipse of 2-D Normal Distribution",
            "From the mean and covariance matrices of a 2-D normal distribution, calculate an expression for the region over which X% of the data is scattered (error ellipse)."
        ),
        Route::StatRocAucCiEn => (
            "Confidence interval of ROC-AUC",
            "From the AUC of the ROC curve and the number of samples, the range of values that the AUC can take (confidence interval) is calculated."
        ),
        Route::UnitHomeEn => (
            "Conversion of Units",
            ""
        ),
        Route::UnitLengthEn => (
            "Interconversion of Length",
            "Converts length units into each other. In addition to metric, yard-pound and Japanese (Shaku-Kan) units, it also supports maritime units."
        ),
        Route::UnitMassEn => (
            "Interconversion of Mass",
            "Converts mass units into each other. Supports metric, yard-pound and Japanese (Shaku-Kan) units."
        ),
        Route::PrivacyEn => (
            "Privacy Policy / Disclaimer",
            ""
        ),

        Route::NotFound => (
            "404",
            "Page Not Found."
        )
    };
    (title.to_string(), dscr.to_string())
}