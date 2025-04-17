use coordinate::functions::circle_center;
use coordinate::point::LatLon;
use yew::prelude::*;

use crate::announce::{InCaseDistortion, InvalidInput};
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_core;
use crate::{parse_state, set_lang};
use crate::router::{Lang, Route};
use crate::title::{Thumbnail, Title};
use crate::utils::onchange_form;

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
        <th span="row">{name}</th>
        <td><input type="number" step=0.0000001 value={value_lat} onchange={onchange_lat} class="form-control" id={id_lat} /></td>
        <td><input type="number" step=0.0000001 value={value_lon} onchange={onchange_lon} class="form-control" id={id_lon} /></td>
        </tr>
    }
}

set_lang!(_parameter, "同一距離にある地点", "Points at the Same Distance");
set_lang!(_result, "計算結果", "Results");
set_lang!(_lat, "緯度", "Latitude");
set_lang!(_lon, "経度", "Longitude");
set_lang!(_centre, "中心地点", "Centre Point");
set_lang!(_point, "地点", "Point");
set_lang!(_calc, "計算", "Calc");
set_lang!(_mean, "平均", "Mean");
set_lang!(_std, "標準偏差", "Standard Deviation");
set_lang!(_dist, "中心点からの距離", "Distance from Centre Point");

#[function_component(MapCircleCenter)]
pub fn map_circle_center(props: &HomeProps) -> Html {
    let lang = props.lang;
    let lat_1 = use_state(|| 34.4549588.to_string());
    let lon_1 = use_state(|| 136.7251689.to_string());
    let lat_2 = use_state(|| 33.8406465.to_string());
    let lon_2 = use_state(|| 135.7736686.to_string());
    let lat_3 = use_state(|| 34.4599703.to_string());
    let lon_3 = use_state(|| 134.8524699.to_string());
    let lat_c = use_state(|| "0".to_string());
    let lon_c = use_state(|| "0".to_string());
    let mean = use_state(|| "0".to_string());
    let std = use_state(|| "0".to_string());

    let onclick = {
        let lat_1 = lat_1.clone();
        let lon_1 = lon_1.clone();
        let lat_2 = lat_2.clone();
        let lon_2 = lon_2.clone();
        let lat_3 = lat_3.clone();
        let lon_3 = lon_3.clone();
        let lat_c = lat_c.clone();
        let lon_c = lon_c.clone();
        let mean = mean.clone();
        let std = std.clone();
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
            let d0 = match center.direction_plane(p0) { Ok((d0, _)) => d0, Err(_) => return, };
            let d1 = match center.direction_plane(p1) { Ok((d1, _)) => d1, Err(_) => return, };
            let d2 = match center.direction_plane(p2) { Ok((d2, _)) => d2, Err(_) => return, };
            let m = (d0 + d1 + d2) / 3.0;
            let s = (((d0-m)*(d0-m) + (d1-m)*(d1-m) + (d2-m)*(d2-m)) / 3.0).sqrt();
            lat_c.set(center.lat().deg().to_string());
            lon_c.set(center.lon().deg().to_string());
            mean.set(m.to_string());
            std.set(s.to_string());
        })
    };

    html! {
        <>
        <Header route={Route::MapCircleCenter} {lang} />
        <BreadCrumb route={Route::MapCircleCenter} {lang} />
        <main class="container mt-2">
        <Title route={Route::MapCircleCenter} {lang} />
        <Thumbnail img={"/img/circle_centre.png"} />
        <InvalidInput {lang} />
        <InCaseDistortion {lang} />
        <div class="row justify-content-md-center">
        <div class={class_core("")}>
        <div class="d-none d-sm-block">
        <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="3">{_parameter(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="col" style="width: 20%"></th>
                    <th scope="col" style="width: 40%">{_lat(lang)}</th>
                    <th scope="col" style="width: 40%">{_lon(lang)}</th>
                </tr>
                <MapCircleCenterForm
                    point={_point(lang)} id="1" lat={lat_1.clone()} lon={lon_1.clone()}
                    onchange_lat={onchange_form(lat_1.clone())} onchange_lon={onchange_form(lon_1.clone())}
                />
                <MapCircleCenterForm
                    point={_point(lang)} id="2" lat={lat_2.clone()} lon={lon_2.clone()}
                    onchange_lat={onchange_form(lat_2.clone())} onchange_lon={onchange_form(lon_2.clone())}
                />
                <MapCircleCenterForm
                    point={_point(lang)} id="3" lat={lat_3.clone()} lon={lon_3.clone()}
                    onchange_lat={onchange_form(lat_3.clone())} onchange_lon={onchange_form(lon_3.clone())}
                />
                <tr><td colspan="3">
                    <div class="d-grid gap-2">
                        <button type="submit" onclick={onclick.clone()} class="btn btn-primary">{_calc(lang)}</button>
                    </div>
                </td></tr>
            </tbody>
        </table>
        </div>
        <div class="d-sm-none">
        <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="2">{_parameter(lang)}</th></tr>
            </thead>
            <tbody>
                <tr><th scope="col" colspan="2" class="text-center">{_point(lang) + " 1"}</th></tr>
                <tr>
                    <th scope="row">{_lat(lang)}</th>
                    <td>
                        <input type="number" step=0.0000001 value={(*lat_1).clone()}
                        onchange={onchange_form(lat_1.clone())} class="form-control" id={"lat_1"} />
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_lon(lang)}</th>
                    <td>
                        <input type="number" step=0.0000001 value={(*lon_1).clone()}
                        onchange={onchange_form(lon_1.clone())} class="form-control" id={"lon_1"} />
                    </td>
                </tr>
                <tr><th scope="col" colspan="2" class="text-center">{_point(lang) + " 2"}</th></tr>
                <tr>
                    <th scope="row">{_lat(lang)}</th>
                    <td>
                        <input type="number" step=0.0000001 value={(*lat_2).clone()}
                        onchange={onchange_form(lat_2.clone())} class="form-control" id={"lat_2"} />
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_lon(lang)}</th>
                    <td>
                        <input type="number" step=0.0000001 value={(*lon_2).clone()}
                        onchange={onchange_form(lon_2.clone())} class="form-control" id={"lon_2"} />
                    </td>
                </tr>
                <tr><th scope="col" colspan="2" class="text-center">{_point(lang) + " 3"}</th></tr>
                <tr>
                    <th scope="row">{_lat(lang)}</th>
                    <td>
                        <input type="number" step=0.0000001 value={(*lat_3).clone()}
                        onchange={onchange_form(lat_3.clone())} class="form-control" id={"lat_3"} />
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_lon(lang)}</th>
                    <td>
                        <input type="number" step=0.0000001 value={(*lon_3).clone()}
                        onchange={onchange_form(lon_3.clone())} class="form-control" id={"lon_3"} />
                    </td>
                </tr>
                <tr><td colspan="2">
                    <div class="d-grid gap-2">
                        <button type="submit" onclick={onclick} class="btn btn-primary">{_calc(lang)}</button>
                    </div>
                </td></tr>
            </tbody>
        </table>
        </div>
        <div class="d-none d-sm-block">
        <table class="table align-middle mt-5">
            <thead>
                <tr><th scope="col" colspan="3">{_result(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="col"></th>
                    <th scope="col" colspan="2">{_lat(lang)}</th>
                    <th scope="col" colspan="2">{_lon(lang)}</th>
                </tr>
                <tr>
                    <th scope="row">{_centre(lang)}</th>
                    <td colspan="2"><input type="text" value={(*lat_c).clone()} class="form-control" id="lat_c" /></td>
                    <td colspan="2"><input type="text" value={(*lon_c).clone()} class="form-control" id="lon_c" /></td>
                </tr>
                <tr>
                    <th scope="col"></th>
                    <th scope="col" colspan="2">{_mean(lang)}</th>
                    <th scope="col" colspan="2">{_std(lang)}</th>
                </tr>
                <tr>
                    <th style="width: 20%"  scope="row">{_dist(lang)}</th>
                    <td style="width: 35%" ><input type="text" value={(*mean).clone()} class="form-control" id="mean" /></td>
                    <td style="width: 5%" >{"m"}</td>
                    <td style="width: 35%" ><input type="text" value={(*std).clone()} class="form-control" id="std" /></td>
                    <td style="width: 5%" >{"m"}</td>
                </tr>
            </tbody>
        </table>
        </div>
        <div class="d-sm-none">
        <table class="table align-middle mt-5">
            <thead>
                <tr><th scope="col" colspan="3">{_result(lang)}</th></tr>
            </thead>
            <tbody>
                <tr><th scope="col" colspan="3" class="text-center">{_centre(lang)}</th></tr>
                <tr>
                    <th scope="row">{_lat(lang)}</th>
                    <td colspan="2">
                        <input type="text" value={(*lat_c).clone()} class="form-control" id="lat_c" />
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_lon(lang)}</th>
                    <td colspan="2">
                        <input type="text" value={(*lon_c).clone()} class="form-control" id="lon_c" />
                    </td>
                </tr>
                <tr><th scope="col" colspan="3" class="text-center">{_dist(lang)}</th></tr>
                <tr>
                    <th scope="row">{_mean(lang)}</th>
                    <td>
                        <input type="text" value={(*mean).clone()} class="form-control" id="mean" />
                    </td>
                    <td>{"m"}</td>
                </tr>
                <tr>
                    <th scope="row">{_std(lang)}</th>
                    <td>
                        <input type="text" value={(*std).clone()} class="form-control" id="std" />
                    </td>
                    <td>{"m"}</td>
                </tr>
            </tbody>
        </table>
        </div>
        </div>
        </div>
        </main>
        <Footer {lang} />
        </>
    }
}