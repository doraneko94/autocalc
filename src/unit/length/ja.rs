use yew::prelude::*;

use crate::header::ja::Header;
use crate::unit::length::UnitLengthBase;

#[function_component(UnitLength)]
pub fn unit_length() -> Html {
    html! {
        <>
        <Header />
        <UnitLengthBase
            title="長さの単位を相互変換" lead="なんとかかんとか"
            si_unit="SI単位" yard_pound="ヤード・ポンド法（国際ヤード）" shaku_kan="尺貫法" maritime="海事系"
            meter="メートル (m)"
            inch="インチ (in)" feet="フィート (ft)" yard="ヤード (yd)" chain="チェーン" furlong="ハロン (fur)" mile="マイル (mi)"
            sun="寸" shaku="尺" ken="間・歩・尋" jo="丈" cho="町" ri="里"
            kairi="海里" fathom="ファゾム"
        />
        </>
    }
}