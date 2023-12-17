use yew::prelude::*;

use crate::header::en::Header;
use crate::map::circle_center::MapCircleCenterBase;

#[function_component(MapCircleCenter)]
pub fn map_circle_center() -> Html {
    html! {
        <>
        <Header />
        <MapCircleCenterBase
            title="Latitude/Longitude of Points Equidistant from the Three Points"
            lat="Latitude" lon="Longitude" center="Centre Point"
            point="Point" calc="Calc"
        />
        </>
    }
}