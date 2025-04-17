use matrix::definite::{Def, is_def};
use statrs::distribution::{ChiSquared, ContinuousCDF};
use trigo::angle::Angle;
use yew::prelude::*;

use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_core;
use crate::{parse_state, set_lang};
use crate::announce::{InvalidInput, Reference};
use crate::router::{Lang, Route};
use crate::title::{Thumbnail, Title};
use crate::utils::onchange_form;

set_lang!(_parameter, "データの形状と誤差楕円の範囲", "Shape of Data and the Range of Error");
set_lang!(_mean, "平均", "Mean");
set_lang!(_cov, "共分散行列", "Covariance Matrix");
set_lang!(_range, "誤差の範囲", "Range of Error");
set_lang!(_calc, "計算", "Calc");
set_lang!(_result, "計算結果", "Results");
set_lang!(_centre, "楕円の中心", "");
set_lang!(_major, "長軸の長さ", "Length of Major Axis");
set_lang!(_minor, "短軸の長さ", "Length of Minor Axis");
set_lang!(_angle, "楕円の傾き角", "Elliptical Tilt Angle");

#[function_component(StatErrorEllipse)]
pub fn stat_error_ellipse(props: &HomeProps) -> Html {
    let lang = props.lang;

    let mx = use_state(|| 8.to_string());
    let my = use_state(|| 12.to_string());
    let sx = use_state(|| 16.to_string());
    let sxy = use_state(|| (8.83176).to_string());
    let sy = use_state(|| 9.to_string());
    let percentile = use_state(|| 95.to_string());
    let major = use_state(|| 0.to_string());
    let minor = use_state(|| 0.to_string());
    let angle = use_state(|| 0.to_string());

    let onclick = {
        let sx = sx.clone();
        let sxy = sxy.clone();
        let sy = sy.clone();
        let percentile = percentile.clone();
        let major = major.clone();
        let minor = minor.clone();
        let angle = angle.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let sx = parse_state!(sx, f64);
            let sxy = parse_state!(sxy, f64);
            let sy = parse_state!(sy, f64);
            let x = parse_state!(percentile, f64) / 100.0;
            if is_def(sx, sxy, sxy, sy) <= Def::Non || x <= 0.0 || x >= 1.0 {
                return;
            }
            let p = (sx + sy) / 2.0;
            let q = ((sx - sy)*(sx - sy) + 4.0*sxy*sxy).sqrt() / 2.0;
            let su = (p + q).sqrt();
            let sv = (p - q).sqrt();

            let chi2 = ChiSquared::new(2.0).unwrap();
            let c = chi2.inverse_cdf(x).sqrt();

            major.set((2.0 * c * su).to_string());
            minor.set((2.0 * c * sv).to_string());
            angle.set(Angle::atan((su*su - sx) / sxy).deg().to_string());
        })
    };

    html! {
        <>
        <Header route={Route::StatErrorEllipse} {lang} />
        <BreadCrumb route={Route::StatErrorEllipse} {lang} />
        <main class="container mt-2">
            <Title route={Route::StatErrorEllipse} {lang} />
            <Thumbnail img={"/img/ellipse.webp"} />
            <InvalidInput {lang} />
            <Reference {lang} url_ja={"https://ushitora.net/archives/3733"} url_en={"https://ushitora.net/3733"} />
            <div class="row justify-content-md-center">
            <div class={class_core("")}>
            <div class="d-none d-md-block">
            <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="6">{_parameter(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row" style="width: 40%">{_mean(lang)}</th>
                    <td style="width: 4%">{"("}</td>
                    <td style="width: 25%">
                        <input type="number" step=0.1 value={(*mx).clone()} onchange={onchange_form(mx.clone())} class="form-control" id="mx" />
                    </td>
                    <td style="width: 2%">{","}</td>
                    <td style="width: 25%">
                        <input type="number" step=0.1 value={(*my).clone()} onchange={onchange_form(my.clone())} class="form-control" id="my" />
                    </td>
                    <td style="width: 4%">{")"}</td>
                </tr>
                <tr>
                    <th scope="row" rowspan="2">{_cov(lang)}</th>
                    <td class="fs-1" rowspan="2">{"["}</td>
                    <td>
                        <input type="number" step=0.01 value={(*sx).clone()} onchange={onchange_form(sx.clone())} class="form-control" id="sx" />
                    </td>
                    <td>{","}</td>
                    <td>
                        <input type="number" step=0.01 value={(*sxy).clone()} onchange={onchange_form(sxy.clone())} class="form-control" id="sxy" />
                    </td>
                    <td class="fs-1" rowspan="2">{"]"}</td>
                </tr>
                <tr>
                    <td>
                        <input type="number" step=0.01 value={(*sxy).clone()} onchange={onchange_form(sxy.clone())} class="form-control" id="syx" />
                    </td>
                    <td>{","}</td>
                    <td>
                        <input type="number" step=0.01 value={(*sy).clone()} onchange={onchange_form(sy.clone())} class="form-control" id="sy" />
                    </td>
                </tr>
                <tr>
                    <th scope="row" colspan="4">{_range(lang)}</th>
                    <td>
                        <input type="number" step=0.1 value={(*percentile).clone()} onchange={onchange_form(percentile.clone())} class="form-control" id="percentile" />
                    </td>
                    <td>{"%"}</td>
                </tr>
                <tr>
                    <td colspan="6">
                    <div class="d-grid gap-2">
                        <button type="submit" onclick={onclick.clone()} class="btn btn-primary">{_calc(lang)}</button>
                    </div>
                    </td>
                </tr>
            </tbody>
            </table>
            </div>
            <div class="d-sm-none">
            <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="5">{_parameter(lang)}</th></tr>
            </thead>
            <tbody>
                <th scope="col" colspan="5" class="text-center">{_mean(lang)}</th>
                <tr>
                    <td style="width: 4%">{"("}</td>
                    <td style="width: 45%">
                        <input type="number" step=0.1 value={(*mx).clone()} onchange={onchange_form(mx.clone())} class="form-control" id="mx" />
                    </td>
                    <td style="width: 2%">{","}</td>
                    <td style="width: 45%">
                        <input type="number" step=0.1 value={(*my).clone()} onchange={onchange_form(my.clone())} class="form-control" id="my" />
                    </td>
                    <td style="width: 4%">{")"}</td>
                </tr>
                <th scope="col" colspan="5" class="text-center">{_cov(lang)}</th>
                <tr>
                    <td class="fs-1" rowspan="2">{"["}</td>
                    <td>
                        <input type="number" step=0.01 value={(*sx).clone()} onchange={onchange_form(sx.clone())} class="form-control" id="sx" />
                    </td>
                    <td>{","}</td>
                    <td>
                        <input type="number" step=0.01 value={(*sxy).clone()} onchange={onchange_form(sxy.clone())} class="form-control" id="sxy" />
                    </td>
                    <td class="fs-1" rowspan="2">{"]"}</td>
                </tr>
                <tr>
                    <td>
                        <input type="number" step=0.01 value={(*sxy).clone()} onchange={onchange_form(sxy.clone())} class="form-control" id="syx" />
                    </td>
                    <td>{","}</td>
                    <td>
                        <input type="number" step=0.01 value={(*sy).clone()} onchange={onchange_form(sy.clone())} class="form-control" id="sy" />
                    </td>
                </tr>
                <tr>
                    <th scope="row" colspan="3">{_range(lang)}</th>
                    <td>
                        <input type="number" step=0.1 value={(*percentile).clone()} onchange={onchange_form(percentile.clone())} class="form-control" id="percentile" />
                    </td>
                    <td>{"%"}</td>
                </tr>
                <tr>
                    <td colspan="5">
                    <div class="d-grid gap-2">
                        <button type="submit" onclick={onclick} class="btn btn-primary">{_calc(lang)}</button>
                    </div>
                    </td>
                </tr>
            </tbody>
            </table>
            </div>

            <table class="table align-middle mt-5">
            <thead>
                <tr><th scope="col" colspan="6">{_result(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <td style="width: 40%">{_mean(lang)}</td>
                    <td style="width: 4%">{"("}</td>
                    <td style="width: 25%">
                        <input type="text" value={(*mx).clone()} class="form-control" id="cx" />
                    </td>
                    <td style="width: 2%">{","}</td>
                    <td style="width: 25%">
                        <input type="text" value={(*my).clone()} class="form-control" id="cy" />
                    </td>
                    <td style="width: 4%">{")"}</td>
                </tr>
                <tr>
                    <td colspan="2">{_major(lang)}</td>
                    <td colspan="3">
                        <input type="text" value={(*major).clone()} class="form-control" id="major" />
                    </td>
                </tr>
                <tr>
                    <td colspan="2">{_major(lang)}</td>
                    <td colspan="3">
                        <input type="text" value={(*minor).clone()} class="form-control" id="minor" />
                    </td>
                </tr>
                <tr>
                    <td colspan="2">{_angle(lang)}</td>
                    <td colspan="3">
                        <input type="text" value={(*angle).clone()} class="form-control" id="angle" />
                    </td>
                    <td>{"°"}</td>
                </tr>
            </tbody>
            </table>
            </div>
            </div>
        </main>
        <Footer {lang} />
        </>
    }
}