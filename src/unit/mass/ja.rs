use yew::prelude::*;

use crate::unit_mass;
use crate::header::ja::Header;
use crate::unit::mass::{
    KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN, KAN,
    UnitMassForm
};

unit_mass!(
    "質量の単位を相互変換",
    "SI単位", "ヤード・ポンド法（帝国単位）", "尺貫法",
    "キログラム (kg)",
    "グレーン (gr)", "ドラム (dr)", "オンス (oz)", "ポンド (lb)", "ストーン (stone)", 
    "クォーター", "センタル", "ハンドレッドウェイト (cwt)", "トン (t)",
    "匁", "両", "斤", "貫"
);