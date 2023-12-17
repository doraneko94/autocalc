use yew::prelude::*;

use crate::header::en::Header;
use crate::unit::mass::UnitMassBase;


#[function_component(UnitMass)]
pub fn unit_mass() -> Html {
    html! {
        <>
        <Header />
        <UnitMassBase
            title="Interconversion of Mass" lead="Example text"
            si_unit="SI unit" yard_pound="Yard & Pound system (British Imperial)" shaku_kan="Japanese Units"
            kg="Kilogram (kg)"
            grain="Grain (gr)" dram="Dram (dr)" ounce="Ounce (oz)" pound="Pound (lb)" stone="Stone (stone)"
            quarter="Quarter" cental="Cental" hrw="Hundredweight (cwt)" ton="Ton (t)"
            monme="匁 -Monme-" ryo="両 -Ryo-" kin="斤 -Kin-" kan="貫 -Kan-"
        />
        </>
    }
}