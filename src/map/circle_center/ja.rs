use yew::prelude::*;

use crate::header::ja::Header;
use crate::map::circle_center::MapCircleCenterBase;

#[function_component(MapCircleCenter)]
pub fn map_circle_center() -> Html {
    html! {
        <>
        <Header />
        <MapCircleCenterBase
            title="３地点から等距離にある地点の緯度経度を計算"
            lat="緯度" lon="経度" center="中心地点"
            point="地点" calc="計算"
        />
        </>
    }
}