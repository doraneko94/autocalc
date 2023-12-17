use yew::prelude::*;

use crate::header::en::Header;
use crate::unit::length::UnitLengthBase;

#[function_component(UnitLength)]
pub fn unit_length() -> Html {
    html! {
        <>
        <Header />
        <UnitLengthBase
            title="Interconversion of Length" lead="Example text"
            si_unit="SI unit" yard_pound="Yard & Pound system (International Yard)" shaku_kan="Japanese Units" maritime="Maritime system"
            meter="Metre (m)"
            inch="Inch (in)" feet="Feet (ft)" yard="Yard (yd)" chain="Chain" furlong="Furlong (fur)" mile="Mile (mi)"
            sun="寸 -Sun-" shaku="尺 -Shaku-" ken="間 -Ken-  歩-Bu-  尋-Hiro-" jo="丈 -Jo-" cho="町 -Cho-" ri="里 -Ri-"
            kairi="Nautical mile" fathom="Fathom"
        />
        </>
    }
}