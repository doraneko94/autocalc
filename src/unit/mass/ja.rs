use yew::prelude::*;

use crate::breadcrumb::BreadCrumb;
use crate::header::ja::Header;
use crate::unit::mass::UnitMassBase;


#[function_component(UnitMass)]
pub fn unit_mass() -> Html {
    html! {
        <>
        <Header />
        <BreadCrumb />
        <UnitMassBase
            title="質量の単位を相互変換" lead="なんとかかんとか"
            si_unit="SI単位" yard_pound="ヤード・ポンド法（帝国単位）" shaku_kan="尺貫法"
            kg="キログラム (kg)"
            grain="グレーン (gr)" dram="ドラム (dr)" ounce="オンス (oz)" pound="ポンド (lb)" stone="ストーン (stone)"
            quarter="クォーター" cental="センタル" hrw="ハンドレッドウェイト (cwt)" ton="トン (t)"
            monme="匁" ryo="両" kin="斤" kan="貫"
        />
        </>
    }
}