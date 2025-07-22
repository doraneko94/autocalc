use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::announce::{InvalidInput, Reference};
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::{class_core, class_half};
use crate::router::{Lang, Route};
use crate::{parse_state, set_lang};
use crate::title::Title;

set_lang!(_none, "無", "None");
set_lang!(_black, "黒", "Black");
set_lang!(_, "", "");

#[derive(Properties, PartialEq)]
pub struct ElectronicDeltaYFormProps {
    pub id: String,
    pub value: UseStateHandle<String>,
    pub name: String,
    pub onchange: Callback<(String, String)>,
}

#[function_component(ElectronicDeltaYForm)]
pub fn electronic_delta_y_form(props: &ElectronicDeltaYFormProps) -> Html {
    let onchange = {
        let state = props.value.clone();
        let id = props.id.clone();
        let onchange_parent = props.onchange.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            state.set(value.clone());
            onchange_parent.emit((value, id.clone()));
        })
    };
    html! {
        <>
            <td class="text-end" style="width: 20%">{props.name.clone()}</td>
            <td style="width: 60%">
                <input type="number" step="0.1" value={(*props.value).clone()} {onchange} class="form-control" id={props.id.clone()} />
            </td>
            <td style="width: 20%">{"\\(\\Omega\\)"}</td>
        </>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = typesetPromise)]
    fn typeset_promise();
}

enum ColorCode {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Gray,
    White,
    Gold,
    Silver,
}

enum I8Mode {
    First,
    Second,
    Third,
    Exp,
}

#[derive(Clone)]
struct Resistance {
    pub first: Option<i8>,
    pub second: Option<i8>,
    pub third: Option<i8>,
    pub exp: Option<i8>,
    pub error: String,
}

impl Default for Resistance {
    fn default() -> Self {
        Self {
            first: Some(0),
            second: Some(0),
            third: None,
            exp: Some(0),
            error: "1".to_string()
        }
    }
}

impl Resistance {
    fn decode_resistance(code: ColorCode) -> Option<u8> {
        match code {
            ColorCode::Black => Some(0),
            ColorCode::Brown => Some(1),
            ColorCode::Red => Some(2),
            ColorCode::Orange => Some(3),
            ColorCode::Yellow => Some(4),
            ColorCode::Green => Some(5),
            ColorCode::Blue => Some(6),
            ColorCode::Purple => Some(7),
            ColorCode::Gray => Some(8),
            ColorCode::White => Some(9),
            ColorCode::Gold => None,
            ColorCode::Silver => None,
        }
    }
}

fn select_resistance<T: Fn(I8Mode)-> Callback<Event>>(f: T, mode: I8Mode) -> Html {
    html! {
        <select class="form-select w-auto" onchange={f(mode)}>
            { match mode {
                I8Mode::Third => html! { <option value="none">{}</> },
                _ => html! { <></> },
            } }

        </select>
    }
}

#[function_component(ElectronicResister)]
pub fn electronic_resister(props: &HomeProps) -> Html {
    let lang = props.lang;
    let resistance = use_state(Resistance::default);
    
    let onchange_i8 = |mode: I8Mode| {
        let resistance = resistance.clone();
        Callback::from(move |e: Event| {
            let mut r = (*resistance).clone();
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = match input.value().parse::<i8>() {
                Ok(i) => Some(i),
                Err(_) => None
            };
            match mode {
                I8Mode::First => { r.first = value; },
                I8Mode::Second => { r.second = value; },
                I8Mode::Third => { r.third = value; },
                I8Mode::Exp => { r.exp = value; },
            }
            resistance.set(r);
        }) 
    };

    let onchange_error = {
        let resistance = resistance.clone();
        Callback::from(move |e: Event| {
            let mut r = (*resistance).clone();
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            r.error = input.value();
            resistance.set(r);
        })
    };

    html! {
        <>
        <Header route={Route::ElectronicDeltaY} {lang} />
        <BreadCrumb route={Route::ElectronicDeltaY} {lang} />
        <main class="container mt-2">
        <Title route={Route::ElectronicDeltaY} {lang} />
        <InvalidInput {lang} />
        <Reference {lang} url_ja={"https://ushitora.net/archives/2744"} url_en={"https://ushitora.net/archives/2744"} />
        <div class="row justify-content-md-center">
            <div class={class_core("")}>
            <table class="table align-middle">
                <thead>
                    <th class="text-center" scope="col" style="width: 50%">{_delta(lang)}</th>
                    <th class="text-center" scope="col" style="width: 50%">{_star(lang)}</th>
                </thead>
                <tbody>
                    <td><img src="/img/D.webp" class="img-fluid" /></td>
                    <td><img src="/img/Y.webp" class="img-fluid" /></td>
                </tbody>
            </table>
            <div class="row justify-content-center">
            <div class={class_half("")}>
            <table class="table align-middle">
            <thead>
                <tr><th class="text-center" scope="col" colspan="3">{_delta(lang)}</th></tr>
            </thead>
            <tbody>
                <tr><ElectronicDeltaYForm id={"d_a"} value={d_a.clone()} name={"\\(R_a\\)"} onchange={onchange.clone()} /></tr>
                <tr><ElectronicDeltaYForm id={"d_b"} value={d_b.clone()} name={"\\(R_b\\)"} onchange={onchange.clone()} /></tr>
                <tr><ElectronicDeltaYForm id={"d_c"} value={d_c.clone()} name={"\\(R_c\\)"} onchange={onchange.clone()} /></tr>
            </tbody>
            </table>
            </div>
            <div class={class_half("")}>
            <table class="table align-middle">
            <thead>
                <tr><th class="text-center" scope="col" colspan="3">{_star(lang)}</th></tr>
            </thead>
            <tbody>
                <tr><ElectronicDeltaYForm id={"s_a"} value={s_a.clone()} name={"\\(r_a\\)"} onchange={onchange.clone()} /></tr>
                <tr><ElectronicDeltaYForm id={"s_b"} value={s_b.clone()} name={"\\(r_b\\)"} onchange={onchange.clone()} /></tr>
                <tr><ElectronicDeltaYForm id={"s_c"} value={s_c.clone()} name={"\\(r_c\\)"} onchange={onchange.clone()} /></tr>
            </tbody>
            </table>
            </div>
            </div>
            </div>
        </div>
        </main>
        <Footer {lang} />
        </>
    }
}