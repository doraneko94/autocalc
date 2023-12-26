use diffeq::solution_int::solve;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::announce::{InvalidInput, OnlyInteger, Reference};
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::layout::class_core;
use crate::{parse_state, set_lang};
use crate::router::parse_query;
use crate::title::Title;
use crate::url::{self, DataMode, Lang};
use crate::utils::onchange_form;
use wasm_bindgen::prelude::*;

set_lang!(_parameter, "微分方程式の係数を入力", "Enter the Coefficients of the Differential Equation");
set_lang!(_calc, "方程式を解く", "Solve");
set_lang!(_result, "微分方程式の一般解", "General Solution of the Differential Equations");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = updateMathjax)]
    fn updateMathjax(id: &str, text: &str);
    #[wasm_bindgen(js_name = typesetPromise)]
    fn typeset_promise();
}

#[function_component(MathDiffeqLinear2Frac)]
pub fn math_diffeq_linear2_frac() -> Html {
    let lang = match parse_query(use_location().unwrap().query_str()).1 {
        Some(Lang::Ja) => Lang::Ja, _ => Lang::En
    };
    let a = use_state(|| 1.to_string());
    let b = use_state(|| 2.to_string());
    let c = use_state(|| (-3).to_string());

    let onclick = {
        let a = a.clone();
        let b = b.clone();
        let c = c.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let a_int = parse_state!(a, isize);
            let b_int = parse_state!(b, isize);
            let c_int = parse_state!(c, isize);
            if parse_state!(a, f64) as isize != a_int { return; }
            if parse_state!(b, f64) as isize != b_int { return; }
            if parse_state!(c, f64) as isize != c_int { return; }
            match solve(a_int, b_int, c_int) {
                Ok(ans) => {
                    let s = "\\(".to_string() + &ans + "\\)";
                    updateMathjax("mathjax-update", &s);
                }
                Err(_) => {}
            }
        })
    };

    use_effect(move || { typeset_promise() });

    html! {
        <>
        <Header />
        <BreadCrumb />
        <main class="container mt-2">
            <Title title={url::math_diffeq_linear2_frac(DataMode::Name(lang))} lead={url::math_diffeq_linear2_frac(DataMode::Dscr(lang))} />
            <InvalidInput />
            <OnlyInteger />
            <Reference url_ja="https://ushitora.net/archives/3400" url_en="https://ushitora.net/en-GB/archives/4532" />
            <div class="row justify-content-md-center">
            <div class={class_core("")}>
            <table class="table align-middle ">
            <thead>
                <tr><th scope="col" colspan="2">{_parameter(lang)}</th></tr>
            </thead>
            <tbody class="text-center">
                <tr><td colspan="2">{"\\(a\\frac{d^2y}{dx^2}+b\\frac{dy}{dx}+cy=0\\)"}</td></tr>
                <tr>
                    <td style="width:30%">{"\\(a\\)"}</td>
                    <td style="width:70%">
                        <input type="number" step=1 value={(*a).clone()} onchange={onchange_form(a.clone())} class="form-control" id="a" />
                    </td>
                </tr>
                <tr>
                    <td>{"\\(b\\)"}</td>
                    <td>
                        <input type="number" step=1 value={(*b).clone()} onchange={onchange_form(b.clone())} class="form-control" id="b" />
                    </td>
                </tr>
                <tr>
                    <td>{"\\(c\\)"}</td>
                    <td>
                        <input type="number" step=1 value={(*c).clone()} onchange={onchange_form(c.clone())} class="form-control" id="c" />
                    </td>
                </tr>
                <tr><td colspan="2">
                    <div class="d-grid gap-2">
                        <button type="submit" onclick={onclick} class="btn btn-primary">{_calc(lang)}</button>
                    </div>
                </td></tr>
            </tbody>
            </table>
            <div class="table-responsive">
            <table class="table text-nowrap align-middle mt-5">
            <thead>
                <tr><th scope="col">{_result(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <p id="mathjax-update"></p>
                </tr>
            </tbody>
            </table>
            </div>
            </div>
            </div>
        </main>
        <Footer />
        </>
    }
}