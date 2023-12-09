use yew::prelude::*;

use crate::unit_length;
use crate::header::en::Header;
use crate::unit::length::{
    METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM,
    UnitLengthForm
};

unit_length!(
    "Interconversion of Length",
    "SI unit", "Yard & Pound system (International Yard)", "Japanese Units", "Maritime system",
    "Metre (m)",
    "Inch (in)", "Feet (ft)", "Yard (yd)", "Chain", "Furlong (fur)", "Mile (mi)",
    "寸 -Sun-", "尺 -Shaku-", "間 -Ken-\n歩 -Bu-\n尋 -Hiro-", "丈 -Joh-", "町 -Cho-", "里 -Ri-",
    "Nautical mile", "Fathom"
);