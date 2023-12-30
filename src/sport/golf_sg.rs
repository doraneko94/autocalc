use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::announce::{InCaseDistortion, InvalidInput};
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::layout::class_core;
use crate::{parse_state, set_lang};
use crate::router::parse_query;
use crate::title::{Thumbnail, Title};
use crate::url::{self, DataMode, Lang};
use crate::unit::length::{FEET, YARD};
use crate::utils::{onchange_form, onchange_select};

set_lang!(_parameter, "ボールの移動", "Movement of the Ball");
set_lang!(_before, "打つ前", "Before Stroke");
set_lang!(_after, "打った後", "After Stroke");
set_lang!(_lie, "ライ", "Lie");
set_lang!(_dist, "カップまでの距離", "Distance to Cup");
set_lang!(_result, "計算結果", "Results");
set_lang!(_ave, "カップインまでの平均打数", "Average Number of Strokes to Cup-In");
set_lang!(_stroke, "打", "strokes");
set_lang!(_expect, "期待値", "Expected value");
set_lang!(_tee, "ティーショット", "Tee Shot");
set_lang!(_fairway, "フェアウェイ", "Fairway");
set_lang!(_rough, "ラフ", "Rough");
set_lang!(_bunker, "バンカー", "Bunker");
set_lang!(_recovery, "リカバリーショット", "Recovery Shot");
set_lang!(_green, "グリーン", "Green");
set_lang!(_calc, "計算", "Calc");

#[function_component(SportGolfSg)]
pub fn sport_golf_sg() -> Html {
    let lang = match parse_query(use_location().unwrap().query_str()).1 {
        Some(Lang::Ja) => Lang::Ja, _ => Lang::En
    };
    let state_before = use_state(|| 0_usize);
    let state_after = use_state(|| 1_usize);
    let dist_before = use_state(|| 500.to_string());
    let dist_after = use_state(|| 200.to_string());
    let unit_before = use_state(|| 0_usize);
    let unit_after = use_state(|| 0_usize);
    let ave_before = use_state(|| 0.to_string());
    let ave_after = use_state(|| 0.to_string());
    let exp_fairway = use_state(|| 0.to_string());
    let exp_rough = use_state(|| 0.to_string());
    let exp_bunker = use_state(|| 0.to_string());
    let exp_recovery = use_state(|| 0.to_string());
    let exp_green = use_state(|| 0.to_string());
    let sg = use_state(|| "".to_string());

    let onclick = {
        let state_before = state_before.clone();
        let state_after = state_after.clone();
        let dist_before = dist_before.clone();
        let dist_after = dist_after.clone();
        let unit_before = unit_before.clone();
        let unit_after = unit_after.clone();
        let ave_before = ave_before.clone();
        let ave_after = ave_after.clone();
        let exp_fairway = exp_fairway.clone();
        let exp_rough = exp_rough.clone();
        let exp_bunker = exp_bunker.clone();
        let exp_recovery = exp_recovery.clone();
        let exp_green = exp_green.clone();
        let sg = sg.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let yd_before = _to_yard(parse_state!(dist_before, f32), *unit_before);
            let yd_after = _to_yard(parse_state!(dist_after, f32), *unit_after);

            let ave_b = _to_ave((*state_before).clone(), yd_before);
            let ave_a = _to_ave((*state_after).clone(), yd_after);
            let shot_gained = ave_b - ave_a - 1.0;
            match lang {
                Lang::Ja => {
                    if shot_gained >= 0.0 { sg.set(format!("{:.2}打の得", shot_gained)); }
                    else { sg.set(format!("{:.2}打の損", -shot_gained)); }
                }
                Lang::En => {
                    if shot_gained >= 0.0 { sg.set(format!("Gain of {:.2} Strokes", shot_gained)); }
                    else { sg.set(format!("Loss of {:.2} Strokes", -shot_gained)); }
                }
            }
            ave_before.set(format!("{:.2}", ave_b));
            ave_after.set(format!("{:.2}", ave_a));
            exp_fairway.set(_exp_fairway(ave_b - 1.0).to_string());
            exp_rough.set(_exp_rough(ave_b - 1.0).to_string());
            exp_bunker.set(_exp_bunker(ave_b - 1.0).to_string());
            exp_recovery.set(_exp_recovery(ave_b - 1.0).to_string());
            exp_green.set(_exp_green(ave_b - 1.0).to_string());
        })
    };

    html! {
        <>
        <Header />
        <BreadCrumb />
        <main class="container mt-2">
        <Title title={url::sport_golf_sg(DataMode::Name(lang))} lead={url::sport_golf_sg(DataMode::Dscr(lang))} />
        //<Thumbnail img={"/img/circle_centre.png"} />
        <InvalidInput />
        <div class="row justify-content-md-center">
        <div class={class_core("")}>
        <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="3">{_parameter(lang)}</th></tr>
            </thead>
            <tbody>
                <tr><th scope="col" colspan="3" class="text-center">{_before(lang)}</th></tr>
                <tr>
                    <th scope="row">{_dist(lang)}</th>
                    <td>
                        <input type="number" step=1 value={(*dist_before).clone()}
                        onchange={onchange_form(dist_before.clone())} class="form-control" id={"dist_before"} />
                    </td>
                    <td>
                        <div>
                        <select class="form-select" aria-label="unit_before" onchange={onchange_select(unit_before.clone())}>
                            <option value=0 selected={*unit_before == 0}>{"yd"}</option>
                            <option value=1 selected={*unit_before == 1}>{"ft"}</option>
                            <option value=2 selected={*unit_before == 2}>{"m"}</option>
                            <option value=3 selected={*unit_before == 3}>{"cm"}</option>
                        </select>
                        </div>
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_lie(lang)}</th>
                    <td>
                        <div colspan="2">
                        <select class="form-select" aria-label="state_before" onchange={onchange_select(state_before.clone())}>
                            <option value=0 selected={*state_before == 0}>{_tee(lang)}</option>
                            <option value=1 selected={*state_before == 1}>{_fairway(lang)}</option>
                            <option value=2 selected={*state_before == 2}>{_rough(lang)}</option>
                            <option value=3 selected={*state_before == 3}>{_bunker(lang)}</option>
                            <option value=4 selected={*state_before == 4}>{_recovery(lang)}</option>
                            <option value=5 selected={*state_before == 5}>{_green(lang)}</option>
                        </select>
                        </div>
                    </td>
                </tr>
                <tr><th scope="col" colspan="3" class="text-center">{_before(lang)}</th></tr>
                <tr>
                    <th scope="row">{_dist(lang)}</th>
                    <td>
                        <input type="number" step=1 value={(*dist_after).clone()}
                        onchange={onchange_form(dist_after.clone())} class="form-control" id={"dist_after"} />
                    </td>
                    <td>
                        <div>
                        <select class="form-select" aria-label="unit_after" onchange={onchange_select(unit_after.clone())}>
                            <option value=0 selected={*unit_after == 0}>{"yd"}</option>
                            <option value=1 selected={*unit_after == 1}>{"ft"}</option>
                            <option value=2 selected={*unit_after == 2}>{"m"}</option>
                            <option value=3 selected={*unit_after == 3}>{"cm"}</option>
                        </select>
                        </div>
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_lie(lang)}</th>
                    <td>
                        <div colspan="2">
                        <select class="form-select" aria-label="state_after" onchange={onchange_select(state_after.clone())}>
                            <option value=1 selected={*state_after == 1}>{_fairway(lang)}</option>
                            <option value=2 selected={*state_after == 2}>{_rough(lang)}</option>
                            <option value=3 selected={*state_after == 3}>{_bunker(lang)}</option>
                            <option value=4 selected={*state_after == 4}>{_recovery(lang)}</option>
                            <option value=5 selected={*state_after == 5}>{_green(lang)}</option>
                        </select>
                        </div>
                    </td>
                </tr>
                
                <tr><td colspan="3">
                    <div class="d-grid gap-2">
                        <button type="submit" {onclick} class="btn btn-primary">{_calc(lang)}</button>
                    </div>
                </td></tr>
            </tbody>
        </table>
        <table class="table align-middle mt-5">
            <thead>
                <tr><th scope="col" colspan="3">{_result(lang)}</th></tr>
            </thead>
            <tbody>
                <tr><th scope="col" colspan="3" class="text-center">{_before(lang)}</th></tr>
                <tr>
                    <th scope="row">{_ave(lang)}</th>
                    <td>
                        <input type="text" value={(*ave_before).clone()} class="form-control" id="ave_before" />
                    </td>
                    <td>{_stroke(lang)}</td>
                </tr>
                <tr><th scope="col" colspan="3" class="text-center">{_after(lang)}</th></tr>
                <tr>
                    <th scope="row">{_ave(lang)}</th>
                    <td>
                        <input type="text" value={(*ave_after).clone()} class="form-control" id="ave_after" />
                    </td>
                    <td>{_stroke(lang)}</td>
                </tr>
                <tr><td class="fs-1 text-center" colspan="3">{(*sg).clone()}</td></tr>
                <tr><th class="text-center" scope="col" colspan="3">{_expect(lang)}</th></tr>
                <tr>
                    <th scope="row">{_fairway(lang)}</th>
                    <td>
                        <input type="text" value={(*exp_fairway).clone()} class="form-control" id="exp_fairway" />
                    </td>
                    <td>{"yd"}</td>
                </tr>
                <tr>
                    <th scope="row">{_rough(lang)}</th>
                    <td>
                        <input type="text" value={(*exp_rough).clone()} class="form-control" id="exp_rough" />
                    </td>
                    <td>{"yd"}</td>
                </tr>
                <tr>
                    <th scope="row">{_bunker(lang)}</th>
                    <td>
                        <input type="text" value={(*exp_bunker).clone()} class="form-control" id="exp_bunker" />
                    </td>
                    <td>{"yd"}</td>
                </tr>
                <tr>
                    <th scope="row">{_recovery(lang)}</th>
                    <td>
                        <input type="text" value={(*exp_recovery).clone()} class="form-control" id="exp_recovery" />
                    </td>
                    <td>{"yd"}</td>
                </tr>
                <tr>
                    <th scope="row">{_green(lang)}</th>
                    <td>
                        <input type="text" value={(*exp_green).clone()} class="form-control" id="exp_green" />
                    </td>
                    <td>{"ft"}</td>
                </tr>
            </tbody>
        </table>
        </div>
        </div>
        </main>
        <Footer />
        </>
    }
}

fn _to_yard(length: f32, unit: usize) -> f32 {
    if unit == 0 { length }
    else if unit == 1 { length / 3.0 }
    else if unit == 2 { length / YARD }
    else { length / 100.0 / YARD }
}

fn _ave_tee(yard: f32) -> f32 { 0.00405 * yard + 2.400 }
fn _ave_fairway(yard: f32) -> f32 { 0.00435 * yard + 2.371 }
fn _ave_rough(yard: f32) -> f32 { 0.00430 * yard + 2.578 }
fn _ave_bunker(yard: f32) -> f32 { 0.00495 * yard + 2.622 }
fn _ave_recovery(yard: f32) -> f32 { 0.00406 * yard + 3.116 }
fn _ave_green(yard: f32) -> f32 { 0.39036 * (yard/3.0).ln() + 0.651 }

fn _exp_fairway(stroke: f32) -> f32 { (stroke - 2.371) / 0.00435 }
fn _exp_rough(stroke: f32) -> f32 { (stroke - 2.578) / 0.00430 }
fn _exp_bunker(stroke: f32) -> f32 { (stroke - 2.622) / 0.00495 }
fn _exp_recovery(stroke: f32) -> f32 { (stroke - 3.116) / 0.00406 }
fn _exp_green(stroke: f32) -> f32 { ((stroke - 0.651) / 0.3936).exp() * 3.0 }

fn _to_ave(state: usize, yard: f32) -> f32 {
    if state == 0 { _ave_tee(yard) }
    else if state == 1 { _ave_fairway(yard) }
    else if state == 2 { _ave_rough(yard) }
    else if state == 3 { _ave_bunker(yard) }
    else if state == 4 { _ave_recovery(yard) }
    else { _ave_green(yard) }
}