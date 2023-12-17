use coordinate::functions::circle_center;
use coordinate::point::LatLon;
use yew::prelude::*;

use crate::parse_state;
use crate::utils::onchange_form;

pub mod ja;
pub mod en;

#[derive(Properties, PartialEq)]
pub struct MapCircleCenterFormProps {
    pub point: String, pub id: String,
    pub lat: UseStateHandle<String>, pub lon: UseStateHandle<String>,
    pub onchange_lat: Callback<Event>, pub onchange_lon: Callback<Event>,
}

#[function_component(MapCircleCenterForm)]
pub fn map_circle_center_form(props: &MapCircleCenterFormProps) -> Html {
    let name = props.point.clone() + props.id.clone().as_str();
    let (value_lat, value_lon) = ((*props.lat).clone(), (*props.lon).clone());
    let (id_lat, id_lon) = (
        "lat_".to_string() + props.id.clone().as_str(),
        "lon_".to_string() + props.id.clone().as_str(),
    );
    let (onchange_lat, onchange_lon) = (props.onchange_lat.clone(), props.onchange_lon.clone());

    html! {
        <tr>
        <td>{name}</td>
        <td><input type="number" step=0.0000001 value={value_lat} onchange={onchange_lat} class="form-control" id={id_lat} /></td>
        <td><input type="number" step=0.0000001 value={value_lon} onchange={onchange_lon} class="form-control" id={id_lon} /></td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct MapCircleCenterBaseProps {
    pub title: String,
    pub lat: String, pub lon: String, pub center: String,
    pub point: String, pub calc: String,
}

#[function_component(MapCircleCenterBase)]
pub fn map_circle_center_base(props: &MapCircleCenterBaseProps) -> Html {
    let lat_1 = use_state(|| 34.4706978.to_string());
    let lon_1 = use_state(|| 136.6937732.to_string());
    let lat_2 = use_state(|| 33.8405858.to_string());
    let lon_2 = use_state(|| 135.5446009.to_string());
    let lat_3 = use_state(|| 34.4600537.to_string());
    let lon_3 = use_state(|| 134.8499204.to_string());
    let lat_c = use_state(|| "0".to_string());
    let lon_c = use_state(|| "0".to_string());

    let onclick = {
        let lat_1 = lat_1.clone();
        let lon_1 = lon_1.clone();
        let lat_2 = lat_2.clone();
        let lon_2 = lon_2.clone();
        let lat_3 = lat_3.clone();
        let lon_3 = lon_3.clone();
        let lat_c = lat_c.clone();
        let lon_c = lon_c.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let lat_1 = parse_state!(lat_1, f64);
            let lon_1 = parse_state!(lon_1, f64);
            let lat_2 = parse_state!(lat_2, f64);
            let lon_2 = parse_state!(lon_2, f64);
            let lat_3 = parse_state!(lat_3, f64);
            let lon_3 = parse_state!(lon_3, f64);
            let p0 = LatLon::new(lat_1, lon_1);
            let p1 = LatLon::new(lat_2, lon_2);
            let p2 = LatLon::new(lat_3, lon_3);
            let center = match circle_center(p0, p1, p2) {
                Ok((center, _)) => center,
                Err(_) => return,
            };
            lat_c.set(center.lat().deg().to_string());
            lon_c.set(center.lon().deg().to_string());
        })
    };

    html! {
        <main class="container-fluid mt-2">
        <table class="table">
            <tbody>
                <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{props.lat.clone()}</th>
                    <th scope="col">{props.lon.clone()}</th>
                </tr>
                <MapCircleCenterForm
                    point={props.point.clone()} id="1" lat={lat_1.clone()} lon={lon_1.clone()}
                    onchange_lat={onchange_form(lat_1.clone())} onchange_lon={onchange_form(lon_1.clone())}
                />
                <MapCircleCenterForm
                    point={props.point.clone()} id="2" lat={lat_2.clone()} lon={lon_2.clone()}
                    onchange_lat={onchange_form(lat_2.clone())} onchange_lon={onchange_form(lon_2.clone())}
                />
                <MapCircleCenterForm
                    point={props.point.clone()} id="3" lat={lat_3.clone()} lon={lon_3.clone()}
                    onchange_lat={onchange_form(lat_3.clone())} onchange_lon={onchange_form(lon_3.clone())}
                />
                <tr>
                    <td>{props.center.clone()}</td>
                    <td><input type="text" value={(*lat_c).clone()} class="form-control" id="lat_c" /></td>
                    <td><input type="text" value={(*lon_c).clone()} class="form-control" id="lon_c" /></td>
                </tr>
                <tr>
                    <td>{"#"}</td>
                    <td>{"#"}</td>
                    <td><button type="submit" onclick={onclick} class="btn btn-primary">{props.calc.clone()}</button></td>
                </tr>
            </tbody>
        </table>
        </main>
    }
}