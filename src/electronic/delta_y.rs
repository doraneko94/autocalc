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

set_lang!(_delta, "デルタ回路", "Delta Circuit");
set_lang!(_star, "スター回路", "Star Circuit");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = typesetPromise)]
    fn typeset_promise();
}

#[function_component(ElectronicDeltaY)]
pub fn electronic_delta_y(props: &HomeProps) -> Html {
    let lang = props.lang;
    let d_a = use_state(|| 30.to_string());
    let d_b = use_state(|| 30.to_string());
    let d_c = use_state(|| 30.to_string());
    let s_a = use_state(|| 10.to_string());
    let s_b = use_state(|| 10.to_string());
    let s_c = use_state(|| 10.to_string());
        
    let onchange = {
        let d_a = d_a.clone();
        let d_b = d_b.clone();
        let d_c = d_c.clone();
        let s_a = s_a.clone();
        let s_b = s_b.clone();
        let s_c = s_c.clone();

        Callback::from(move |(value, id): (String, String)| {
            match id.as_str() {
                "d_a" | "d_b" | "d_c" => {
                    let a = if id == "d_a" { parse_state!(value, f32) } else { parse_state!(d_a, f32) };
                    let b = if id == "d_b" { parse_state!(value, f32) } else { parse_state!(d_b, f32) };
                    let c = if id == "d_c" { parse_state!(value, f32) } else { parse_state!(d_c, f32) };
                    let den = a + b + c;
                    s_a.set((b * c / den).to_string());
                    s_b.set((c * a / den).to_string());
                    s_c.set((a * b / den).to_string());
                }
                _ => {
                    let a = if id == "s_a" { parse_state!(value, f32) } else { parse_state!(s_a, f32) };
                    let b = if id == "s_b" { parse_state!(value, f32) } else { parse_state!(s_b, f32) };
                    let c = if id == "s_c" { parse_state!(value, f32) } else { parse_state!(s_c, f32) };
                    let num = a*b + b*c + c*a;
                    d_a.set((num / a).to_string());
                    d_b.set((num / b).to_string());
                    d_c.set((num / c).to_string());
                }
            };
        })
    };
    use_effect(move || { typeset_promise(); });

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