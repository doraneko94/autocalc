use crate::router::Route;

pub fn title_dscr(route: Route) -> (String, String) {
    let (title, dscr) = match route {
        Route::Home => (
            "TOP", 
            "AutoCalcは数字と闘う職人・エンジニアのため、高速かつ高精度の計算をオンラインにて無料で提供します。単位の変換といった日常生活で役立つ計算から、学術論文で活用できる統計関数まで、幅広い計算に対応しています。"
        ),
        Route::Privacy => (
            "プライバシーポリシー・免責事項",
            "AutoCalcのプライバシーポリシー・免責事項です。"
        ),
        Route::DigitalHome => (
            "デジタル計算",
            "コンピュータで数字を扱うときのデータ形式を可視化したり、様々な計算を実行したりします。"
        ),
        Route::DigitalBase => (
            "2進数・8進数・10進数・16進数を相互に変換",
            "2進数・8進数・10進数・16進数のいずれかで入力された数字を、それ以外の進数に自動変換します。自動のゼロ埋めにも対応しています。"
        ),
        Route::DigitalBitCalc => (
            "ビット演算",
            "NOT, AND, OR, NAND, NOR, XOR, XNORに加え、左右へのシフト演算を実行します。"
        ),
        Route::DigitalFloat => (
            "浮動小数点数の変換 (IEEE 754形式)",
            "入力された小数をIEEE 754形式の浮動小数点数に変換し、符号・指数部・仮数部の各ビットを表示します。32ビットと64ビットに対応しています。"
        ),
        Route::ElectronicHome => (
            "電気回路・電子回路",
            "電気回路や電子回路の設計に役立つ計算を提供します。"
        ),
        Route::ElectronicDeltaY => (
            "デルタ回路・スター回路を相互変換",
            "等価なデルタ回路とスター回路に含まれる抵抗値の値をリアルタイムに計算します。", 
        ),
        Route::ElectronicResister => (
            "カラーコードから抵抗値・許容差を計算",
            "抵抗器に書かれたカラーコードから、その抵抗値と許容差を計算します。"
        ),
        Route::MapHome => (
            "地図・空間情報",
            "地図や空間情報を扱うための座標上での計算を提供します。"
        ),
        Route::MapCircleCenter => (
            "３地点から等距離にある地点の緯度経度を計算",
            "３つの地点を設定し、それらから同じ距離にある地点の緯度経度を求めます。"
        ),
        Route::MapDmsFloat => (
            "緯度経度の度分秒と浮動小数点数を相互変換",
            "緯度経度を度・分・秒形式、または小数形式で入力すると、自動的にもう一方の形式に変換します。"
        ),
        Route::MathHome => (
            "数学",
            "近似値を計算したり、方程式の解を求めたりします。"
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
            "各種スポーツで使われる指標を自動計算します。"
        ),
        Route::SportGolfSg => (
            "ゴルフ: 稼いだ打数 (SG)",
            "ストローク前後の球の位置と状況から、ストロークによって稼いだまたは失ったスコアを計算します。"
        ),
        Route::StatHome => (
            "統計計算",
            "高度な関数を用いて、統計検定や確率推定のための重要な指標を計算します。"
        ),
        Route::StatErrorEllipse => (
            "2次元正規分布の誤差楕円",
            "2次元正規分布の平均と共分散行列から、データのX%が散らばる領域（誤差楕円）の式を計算します。"
        ),
        Route::StatRocAucCi => (
            "ROC曲線のAUCがとる信頼区間を計算",
            "ROC曲線のAUCとサンプル数から、AUCが取りうる値の範囲 (信頼区間) を計算します。"
        ),
        Route::UnitHome => (
            "単位変換", 
            "様々な単位を相互に変換し、それぞれの関係をリアルタイムに表示します。"
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
            "AutoCalc provides fast, accurate calculations online for free for craftsmen and engineers who struggle with numbers. It supports a wide range of computations, from calculations useful in everyday life such as unit conversion to statistical functions that can be used in academic papers."
        ),
        Route::DigitalHomeEn => (
            "Digital Computing",
            "These pages show data formats when handling numbers on a computer and perform various calculations."
        ),
        Route::DigitalBaseEn => (
            "Convert Binary, Octal, Decimal, and Hexadecimal",
            "This automatically converts numbers entered in binary, octal, decimal, or hexadecimal notation to other bases. It also supports automatic zero padding."
        ),
        Route::DigitalBitCalcEn => (
            "Bit Calculation",
            "Performs NOT, AND, OR, NAND, NOR, XOR, XNOR, plus left and right shift operations."
        ),
        Route::DigitalFloatEn => (
            "Floating-point conversion (IEEE 754 format)",
            "Converts the input decimal number to a floating-point number in IEEE 754 format and displays the sign, exponent, and mantissa bits. Supports 32-bit and 64-bit."
        ),
        Route::ElectronicHomeEn => (
            "Electrical & Electronic Circuits",
            "Provides calculations to help you design electrical and electronic circuits."
        ),
        Route::ElectronicDeltaYEn => (
            "Mutual Conversion of Delta and Star Circuits",
            "Calculate in real time the values of the resistances contained in equivalent Delta and Star circuits."
        ),
        Route::ElectronicResisterEn => (
            "Calculate Resistance and Tolerance from Color Code",
            "Calculate the resistance value and tolerance based on the color code printed on a resistor."
        ),
        Route::MapHomeEn => (
            "Maps & Spatial Information",
            "Provides coordinate calculations for working with maps and spatial information."
        ),
        Route::MapCircleCenterEn => (
            "Latitude/Longitude of Points Equidistant from the Three Points",
            "Set three field points and find the latitude/longitude of the points at the same distance from them."
        ),
        Route::MapDmsFloatEn => (
            "Convert Latitude and Longitude in Degrees, Minutes, and Deconds to Floating Point Values",
            "Input latitude and longitude in degrees, minutes, and seconds format or decimal format and it will be automatically converted to the other format."
        ),
        Route::MathHomeEn => (
            "Mathematics",
            "Calculates approximate values and solves equations."
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
            "Sports", 
            "Automatically calculates indicators used in various sports."
        ),
        Route::SportGolfSgEn => (
            "Golf: Stroke Gained",
            "Calculates the score gained or lost by a stroke, based on the position and situation of the ball before and after the stroke."
        ),
        Route::StatHomeEn => (
            "Statistics",
            "Use advanced functions to calculate key metrics for statistical tests and probability estimates."
        ),
        Route::StatErrorEllipseEn => (
            "Error Ellipse of 2-D Normal Distribution",
            "From the mean and covariance matrices of a 2-D normal distribution, calculate an expression for the region over which X% of the data is scattered (error ellipse)."
        ),
        Route::StatRocAucCiEn => (
            "Confidence Interval of ROC-AUC",
            "From the AUC of the ROC curve and the number of samples, the range of values that the AUC can take (confidence interval) is calculated."
        ),
        Route::UnitHomeEn => (
            "Unit Conversion",
            "These pages converts between various units and displays their relationships in real time."
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
            "This is Privacy Policy / Disclaimer of AutoCalc."
        ),

        Route::NotFound => (
            "404",
            "Page Not Found."
        )
    };
    (title.to_string(), dscr.to_string())
}