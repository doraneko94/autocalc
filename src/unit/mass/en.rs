use yew::prelude::*;

use crate::unit_mass;
use crate::header::en::Header;
use crate::unit::mass::{
    KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN, KAN,
    UnitMassForm
};

unit_mass!(
    "Interconversion of Mass",
    "SI unit", "Yard & Pound system (British Imperial)", "Japanese Units",
    "Kilogram (kg)",
    "Grain (gr)", "Dram (dr)", "Ounce (oz)", "Pound (lb)", "Stone (stone)", 
    "Quarter", "Cental", "Hundredweight (cwt)", "Ton (t)",
    "匁 -Monme-", "両 -Ryo-", "斤 -Kin-", "貫 -Kan-"
);