use statrs::distribution::{Normal, ContinuousCDF};
use yew::prelude::*;

use crate::announce::{InvalidInput, Reference};
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_core;
use crate::{parse_state, set_lang};
use crate::router::{Lang, Route};
use crate::title::{Thumbnail, Title};
use crate::utils::onchange_form;

set_lang!(_parameter, "AUCとサンプル数を入力", "Enter AUC and Number of Samples");
set_lang!(_auc, "ROC-AUC", "ROC-AUC");
set_lang!(_pos, "陽性サンプル数", "Number of Positive samples");
set_lang!(_neg, "陰性サンプル数", "Number of Negative samples");
set_lang!(_ci, "信頼区間", "Confidence Interval");
set_lang!(_calc, "計算", "Calc");
set_lang!(_result, "計算結果", "Results");
set_lang!(_lower, "AUCの下限値", "Lower bound of AUC");
set_lang!(_upper, "AUCの上限値", "Upper bound of AUC");

#[function_component(StatRocAucCi)]
pub fn stat_roc_auc_ci(props: &HomeProps) -> Html {
    let lang = props.lang;
    let auc = use_state(|| 0.8.to_string());
    let pos = use_state(|| 20.to_string());
    let neg = use_state(|| 20.to_string());
    let percentile = use_state(|| 95.to_string());
    let lower = use_state(|| 0.to_string());
    let upper = use_state(|| 0.to_string());

    let onclick = {
        let auc = auc.clone();
        let pos = pos.clone();
        let neg = neg.clone();
        let percentile = percentile.clone();
        let lower = lower.clone();
        let upper = upper.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let auc = parse_state!(auc, f64);
            let np = parse_state!(pos, f64);
            let nn = parse_state!(neg, f64);
            let x = parse_state!(percentile, f64) / 100.0;
            if auc < 0.0 || auc > 1.0 || np <= 0.0 || nn <= 0.0 || x <= 0.0 || x >= 1.0 {
                return;
            }
            let a = (1.0 - x) / 2.0;

            let norm = Normal::new(0.0, 1.0).unwrap();
            let z = norm.inverse_cdf(x + a);
            let q1 = auc / (2.0 - auc);
            let q2 = 2.0 * auc * auc / (1.0 + auc);
            let se = ((auc*(1.0-auc) + (np-1.0)*(q1-auc*auc) + (nn-1.0)*(q2-auc*auc)) / (np*nn)).sqrt();
            let (mut lo, mut up) = (auc - z * se, auc + z * se);
            if lo < 0.0 { lo = 0.0; }
            if up > 1.0 { up = 1.0; }

            lower.set(lo.to_string());
            upper.set(up.to_string());
        })
    };

    html! {
        <>
        <Header route={Route::StatRocAucCi} {lang} />
        <BreadCrumb route={Route::StatRocAucCi} {lang} />
        <main class="container mt-2">
            <Title route={Route::StatRocAucCi} {lang} />
            <Thumbnail img={"/img/roc_auc.webp"} />
            <InvalidInput {lang} />
            <Reference {lang} url_ja="https://ushitora.net/archives/800" url_en="https://ushitora.net/archives/800" />
            <div class="row justify-content-md-center">
            <div class={class_core("")}>
            <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="3">{_parameter(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <td style="width: 50%">{_auc(lang)}</td>
                    <td style="width: 45%">
                        <input type="number" step=0.01 value={(*auc).clone()} onchange={onchange_form(auc.clone())} class="form-control" id="auc" />
                    </td>
                    <td style="width: 5%"></td>
                </tr>
                <tr>
                    <td>{_pos(lang)}</td>
                    <td><input type="number" step=1 value={(*pos).clone()} onchange={onchange_form(pos.clone())} class="form-control" id="pos" /></td>
                    <td></td>
                </tr>
                <tr>
                    <td>{_neg(lang)}</td>
                    <td><input type="number" step=1 value={(*neg).clone()} onchange={onchange_form(neg.clone())} class="form-control" id="neg" /></td>
                    <td></td>
                </tr>
                <tr>
                    <td>{_ci(lang)}</td>
                    <td><input type="number" step=0.1 value={(*percentile).clone()} onchange={onchange_form(percentile.clone())} class="form-control" id="percentile" /></td>
                    <td>{"%"}</td>
                </tr>
                <tr><td colspan="3">
                    <div class="d-grid gap-2">
                        <button type="submit" onclick={onclick} class="btn btn-primary">{_calc(lang)}</button>
                    </div>
                </td></tr>
            </tbody>
            </table>
            <table class="table align-middle mt-5">
            <thead>
                <tr><th scope="col" colspan="3">{_result(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <td style="width: 50%">{_lower(lang)}</td>
                    <td style="width: 45%">
                        <input type="text" value={(*lower).clone()} class="form-control" id="lower" />
                    </td>
                    <td style="width: 5%"></td>
                </tr>
                <tr>
                    <td style="width: 50%">{_upper(lang)}</td>
                    <td style="width: 45%">
                        <input type="text" value={(*upper).clone()} class="form-control" id="upper" />
                    </td>
                    <td style="width: 5%"></td>
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