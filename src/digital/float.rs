use yew::prelude::*;

use crate::set_lang;
use crate::announce::InvalidInput;
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_core;
use crate::router::{Lang, Route};
use crate::title::Title;

set_lang!(_float, "浮動小数点数", "Floating Point Number");
set_lang!(_precision, "精度", "Precision");
set_lang!(_single, "単精度 (32ビット)", "32bit Float");
set_lang!(_double, "倍精度 (64ビット)", "64bit Float");
set_lang!(_value, "数値", "Value");
set_lang!(_convert, "変換", "Convert");
set_lang!(_result, "変換結果", "Conversion Result");
set_lang!(_sign, "符号", "Sign");
set_lang!(_exp, "指数部", "Exponent");
set_lang!(_frac, "仮数部", "Mantissa");
set_lang!(_full, "全体", "Whole Bits");
set_lang!(_hex, "16進数表示", "Hexadecimal");

#[derive(Clone,Copy, PartialEq, Eq)]
enum Precision {
    Single,
    Double,
}

struct Outputs {
    pub sign: String,
    pub exp: String,
    pub frac: String,
    pub full: String,
    pub hex: String,
}

#[function_component(DigitalFloat)]
pub fn digital_float(props: &HomeProps) -> Html {
    let lang = props.lang;
    let precision = use_state(|| Precision::Single);
    let input_value = use_state(|| String::new());
    let outputs = use_state(|| None::<Outputs>);

    let on_precision_change = {
        let precision = precision.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if input.value() == "single" {
                precision.set(Precision::Single);
            } else {
                precision.set(Precision::Double);
            }
        })
    };

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        }) 
    };

    let on_convert = {
        let input_value = input_value.clone();
        let precision = precision.clone();
        let outputs = outputs.clone();
        Callback::from(move |_: MouseEvent| {
            let val: f64 = match input_value.parse() {
                Ok(v) => v,
                Err(_) => {
                    outputs.set(None);
                    return;
                }
            };
            let result = match *precision {
                Precision::Single => {
                    let bits = (val as f32).to_bits();
                    Outputs {
                        sign: ((bits >> 31) & 0x1).to_string(),
                        exp: format!("{:08b}", (bits >> 23) & 0xff),
                        frac: format!("{:023b}", bits & 0x7fffff),
                        full: format!("{:032b}", bits),
                        hex: format!("0x{:08X}", bits),
                    }
                }
                Precision::Double => {
                    let bits = val.to_bits();
                    Outputs {
                        sign: ((bits >> 63) & 0x1).to_string(),
                        exp: format!("{:011b}", (bits >> 52) & 0x7ff),
                        frac: format!("{:052b}", bits & 0xfffffffffffff),
                        full: format!("{:064b}", bits),
                        hex: format!("0x{:016X}", bits),
                    }
                }
            };
            outputs.set(Some(result));
        })
    };

    html! {
        <>
        <Header route={Route::DigitalFloat} {lang} />
        <BreadCrumb route={Route::DigitalFloat} {lang} />
        <main class="container mt-2">
            <Title route={Route::DigitalFloat} {lang} />
            <InvalidInput {lang} />
            <div class="row justify-content-md-center">
            <div class={class_core("")}>
            <div class="table-responsive">
            <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="3">{_float(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row" style="width: 20%">{_precision(lang)}</th>
                    <td style="width: 40%">
                        <div class="form-check form-check-inline">
                            <input class="form-check-input" type="radio" name="precision" value="single" checked={*precision == Precision::Single} onchange={on_precision_change.clone()} />
                            <label class="form-check-label">{_single(lang)}</label>
                        </div>
                    </td>
                    <td style="width: 40%">
                        <div class="form-check form-check-inline">
                            <input class="form-check-input" type="radio" name="precision" value="double" checked={*precision == Precision::Double} onchange={on_precision_change} />
                            <label class="form-check-label">{_double(lang)}</label>
                        </div>
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_value(lang)}</th>
                    <td colspan="2">
                        <input type="text" class="form-control" value={(*input_value).clone()} oninput={on_input} />
                    </td>
                </tr>
                <tr>
                    <td colspan="3">
                    <div class="d-grid gap-2">
                    <button type="submit" class="btn btn-primary" onclick={on_convert}>{_convert(lang)}</button>
                    </div>
                    </td>
                </tr>
            </tbody>
            </table>
            
            <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="2">{_result(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row" style="width: 20%">{_sign(lang)}</th>
                    <td style="width: 80%">
                        <input type="text" value={match &(*outputs) { Some(result) => result.sign.clone(), None => "".to_string() }} readonly=true class="form-control" />
                    </td>
                </tr>
                <tr>
                    <th scope="row" style="width: 20%">{_exp(lang)}</th>
                    <td style="width: 80%">
                        <input type="text" value={match &(*outputs) { Some(result) => result.exp.clone(), None => "".to_string() }} readonly=true class="form-control" />
                    </td>
                </tr>
                <tr>
                    <th scope="row" style="width: 20%">{_frac(lang)}</th>
                    <td style="width: 80%">
                        <input type="text" value={match &(*outputs) { Some(result) => result.frac.clone(), None => "".to_string() }} readonly=true class="form-control" />
                    </td>
                </tr>
                <tr>
                    <th scope="row" style="width: 20%">{_full(lang)}</th>
                    <td style="width: 80%">
                        <input type="text" value={match &(*outputs) { Some(result) => result.full.clone(), None => "".to_string() }} readonly=true class="form-control" />
                    </td>
                </tr>
                <tr>
                    <th scope="row" style="width: 20%">{_hex(lang)}</th>
                    <td style="width: 80%">
                        <input type="text" value={match &(*outputs) { Some(result) => result.hex.clone(), None => "".to_string() }} readonly=true class="form-control" />
                    </td>
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