use yew::prelude::*;

use crate::unit_length;
use crate::header::ja::Header;
use crate::unit::length::{
    METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM,
    UnitLengthForm
};

unit_length!(
    "長さの単位を相互変換",
    "SI単位", "ヤード・ポンド法（国際ヤード）", "尺貫法", "海事系",
    "メートル (m)",
    "インチ (in)", "フィート (ft)", "ヤード (yd)", "チェーン", "ハロン (fur)", "マイル (mi)",
    "寸", "尺", "間・歩・尋", "丈", "町", "里",
    "海里", "ファゾム"
);