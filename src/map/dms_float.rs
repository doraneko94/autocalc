use std::cmp::min;
use yew::prelude::*;

use crate::announce::InvalidInput;
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_core;
use crate::router::{Lang, Route};
use crate::set_lang;
use crate::title::Title;

struct DMS {
    pub degree: i16,
    pub minute: u8,
    pub second: f64,
}

impl DMS {
    fn new(degree: i16, minute: u8, second: f64) -> Self {
        let degree = degree % 180;
        let minute = min(minute, 59);
        let second = if second >= 60.0 { 60.0 - f64::EPSILON } else { second };
        Self { degree, minute, second }
    }

    fn from_f64(value: f64) -> Self {
        let mut value = value % 180.0;
        let degree = value as i16;
        value = (value.abs() - degree.abs() as f64) * 60.0;
        let minute = value as u8;
        let second = (value - minute as f64) * 60.0;
        Self { degree, minute, second }
    }

    fn to_f64(&self) -> f64 {
        (self.degree as f64 + self.minute as f64 / 60.0 + self.second / 3600.0) % 180.0
    }
}

set_lang!(_lat, "緯度", "Latitude");
set_lang!(_lng, "経度", "Longitude");
set_lang!(_dms, "度分秒", "Degree,Minute,Second");
set_lang!(_dec, "小数", "Decimal");

#[function_component(MapDmsFloat)]
pub fn dms_float(props: &HomeProps) -> Html {
    let lang = props.lang;
    let lat = use_state(|| DMS::from_f64(35.68244087111691));
    let lat_float = use_state(|| lat.to_f64());
    let lng = use_state(|| DMS::from_f64(139.75284969823213));
    let lng_float = use_state(|| lng.to_f64());

    html! {
        <>
        <Header route={Route::MapDmsFloat} {lang} />
        <BreadCrumb route={Route::MapDmsFloat} {lang} />
        <main class="container mt-2">
        <Title route={Route::MapDmsFloat} {lang} />
        <InvalidInput {lang} />
        <div class="row justify-content-md-center">
        <div class={class_core("")}>
        <div class="table-responsive">
        <table class="table align-middle">
        <thead>
            <tr>
            <th scope="col"></th>
            </tr>
        </thead>
        <tbody>
            <tr>
            <th scope="row" style="width: 25%">{format!("{} ({})", _lat(lang), _dms(lang))}</th>
            <td style="width: 20%"><input type="number" class="form-control" value={lat.degree.to_string()} oninput={
                let lat = lat.clone();
                let lat_float = lat_float.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(v) = input.value().parse::<i16>() {
                        let dms = DMS::new(v, lat.minute, lat.second);
                        lat_float.set(dms.to_f64());
                        lat.set(dms);
                    }
                })
            } /></td>
            <td style="width: 5%">{"°"}</td>
            <td style="width: 20%"><input type="number" class="form-control" value={lat.minute.to_string()} oninput={
                let lat = lat.clone();
                let lat_float = lat_float.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(v) = input.value().parse::<u8>() {
                        let dms = DMS::new(lat.degree, v, lat.second);
                        lat_float.set(dms.to_f64());
                        lat.set(dms);
                    }
                })
            } /></td>
            <td style="width: 5%">{"'"}</td>
            <td style="width: 20%"><input type="number" class="form-control" value={lat.second.to_string()} oninput={
                let lat = lat.clone();
                let lat_float = lat_float.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(v) = input.value().parse::<f64>() {
                        let dms = DMS::new(lat.degree, lat.minute, v);
                        lat_float.set(dms.to_f64());
                        lat.set(dms);
                    }
                })
            } /></td>
            <td style="width: 5%">{"\""}</td>
            </tr>

            <tr>
            <th scope="row">{format!("{} ({})", _lat(lang), _dec(lang))}</th>
            <td colspan="6"><input type="number" class="form-control" value={lat_float.to_string()} oninput={
                let lat = lat.clone();
                let lat_float = lat_float.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(v) = input.value().parse::<f64>() {
                        lat_float.set(v);
                        lat.set(DMS::from_f64(v));
                    }
                })
            } /></td>
            </tr>

            <tr>
            <th scope="row">{format!("{} ({})", _lng(lang), _dms(lang))}</th>
            <td><input type="number" class="form-control" value={lng.degree.to_string()} oninput={
                let lng = lng.clone();
                let lng_float = lng_float.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(v) = input.value().parse::<i16>() {
                        let dms = DMS::new(v, lng.minute, lng.second);
                        lng_float.set(dms.to_f64());
                        lng.set(dms);
                    }
                })
            } /></td>
            <td>{"°"}</td>
            <td><input type="number" class="form-control" value={lng.minute.to_string()} oninput={
                let lng = lng.clone();
                let lng_float = lng_float.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(v) = input.value().parse::<u8>() {
                        let dms = DMS::new(lng.degree, v, lng.second);
                        lng_float.set(dms.to_f64());
                        lng.set(dms);
                    }
                })
            } /></td>
            <td>{"'"}</td>
            <td><input type="number" class="form-control" value={lng.second.to_string()} oninput={
                let lng = lng.clone();
                let lng_float = lng_float.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(v) = input.value().parse::<f64>() {
                        let dms = DMS::new(lng.degree, lng.minute, v);
                        lng_float.set(dms.to_f64());
                        lng.set(dms);
                    }
                })
            } /></td>
            <td>{"\""}</td>
            </tr>

            <tr>
            <th scope="row">{format!("{} ({})", _lng(lang), _dec(lang))}</th>
            <td colspan="6"><input type="number" class="form-control" value={lng_float.to_string()} oninput={
                let lng = lng.clone();
                let lng_float = lng_float.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(v) = input.value().parse::<f64>() {
                        lng_float.set(v);
                        lng.set(DMS::from_f64(v));
                    }
                })
            } /></td>
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