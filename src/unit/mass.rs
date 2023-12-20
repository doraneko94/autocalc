use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{parse_state, set_lang};
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::layout::class_core;
use crate::router::parse_query;
use crate::title::Title;
use crate::url::{self, DataMode, Lang};

#[derive(Properties, PartialEq)]
pub struct UnitMassFormProps {
    pub id: String,
    pub value: UseStateHandle<String>,
    pub name: String,
    pub onchange: Callback<(String, String)>,
}

#[function_component(UnitMassForm)]
pub fn unit_mass_form(props: &UnitMassFormProps) -> Html {
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
        <tr>
            <th scope="row" style="width: 50%">{props.name.clone()}</th>
            <td style="width: 50%">
                <input type="number" step="0.1" value={(*props.value).clone()} {onchange} class="form-control" id={props.id.clone()} />
            </td>
        </tr>
    }
}

set_lang!(_si_unit, "SI単位", "SI Unit");
set_lang!(_yard_pound, "ヤード・ポンド法（国際ヤード）", "Yard & Pound system (International Yard)");
set_lang!(_shaku_kan, "尺貫法", "Japanese Units");
set_lang!(_kg, "キログラム (kg)", "Kilogram (kg)");
set_lang!(_grain, "グレーン (gr)", "Grain (gr)");
set_lang!(_dram, "ドラム (dr)", "Dram (dr)");
set_lang!(_ounce, "オンス (oz)", "Ounce (oz)");
set_lang!(_pound, "ポンド (lb)", "Pound (lb)");
set_lang!(_stone, "ストーン (stone)", "Stone (stone)");
set_lang!(_quarter, "クォーター", "Quarter");
set_lang!(_cental, "センタル", "Cental");
set_lang!(_hrw, "ハンドレッドウェイト (cwt)", "Hundredweight (cwt)");
set_lang!(_ton, "トン (t)", "Ton (t)");
set_lang!(_monme, "匁", "匁 -Monme-");
set_lang!(_ryo, "両", "両 -Ryo-");
set_lang!(_kin, "斤", "斤 -Kin-");
set_lang!(_kan, "貫", "貫 -Kan-");

#[function_component(UnitMass)]
pub fn unit_mass() -> Html {
    let lang = match parse_query(use_location().unwrap().query_str()).1 {
        Some(Lang::Ja) => Lang::Ja, _ => Lang::En
    };
    let kg = use_state(|| 1.0_f32.to_string());
    let grain = use_state(|| (1.0 / GRAIN).to_string());
    let dram = use_state(|| (1.0 / DRAM).to_string());
    let ounce = use_state(|| (1.0 / OUNCE).to_string());
    let pound = use_state(|| (1.0 / POUND).to_string());
    let stone = use_state(|| (1.0 / STONE).to_string());
    let quarter = use_state(|| (1.0 / QUARTER).to_string());
    let cental = use_state(|| (1.0 / CENTAL).to_string());
    let hrw = use_state(|| (1.0 / HRW).to_string());
    let ton = use_state(|| (1.0 / TON).to_string());
    let monme = use_state(|| (1.0 / MONME).to_string());
    let ryo = use_state(|| (1.0 / RYO).to_string());
    let kin = use_state(|| (1.0 / KIN).to_string());
    let kan = use_state(|| (1.0 / KAN).to_string());
        
    let onchange = {
        let kg = kg.clone();
        let grain = grain.clone();
        let dram = dram.clone();
        let ounce = ounce.clone();
        let pound = pound.clone();
        let stone = quarter.clone();
        let cental = cental.clone();
        let hrw = hrw.clone();
        let ton = ton.clone();
        let monme = monme.clone();
        let ryo = ryo.clone();
        let kin = kin.clone();
        let kan = kan.clone();

        Callback::from(move |value: (String, String)| {
            let (s, unit) = value;
            let mut kg_f32 = parse_state!(s, f32);
            kg_f32 = match unit.as_str() {
                "kg" => kg_f32,
                "grain" => kg_f32 * GRAIN,
                "dram" => kg_f32 * DRAM,
                "ounce" => kg_f32 * OUNCE,
                "pound" => kg_f32 * POUND,
                "stone" => kg_f32 * STONE,
                "cental" => kg_f32 * CENTAL,
                "hrw" => kg_f32 * HRW,
                "ton" => kg_f32 * TON,
                "monme" => kg_f32 * MONME,
                "ryo" => kg_f32 * RYO,
                "kin" => kg_f32 * KIN,
                "kan" => kg_f32 * KAN,
                _ => 0.0
            };
            if unit.as_str() != "kg" { kg.set(kg_f32.to_string()); }
            if unit.as_str() != "grain" { grain.set((kg_f32 / GRAIN).to_string()); }
            if unit.as_str() != "dram" { dram.set((kg_f32 / DRAM).to_string()); }
            if unit.as_str() != "ounce" { ounce.set((kg_f32 / OUNCE).to_string()); }
            if unit.as_str() != "pound" { pound.set((kg_f32 / POUND).to_string()); }
            if unit.as_str() != "stone" { stone.set((kg_f32 / STONE).to_string()); }
            if unit.as_str() != "cental" { cental.set((kg_f32 / CENTAL).to_string()); }
            if unit.as_str() != "hrw" { hrw.set((kg_f32 / HRW).to_string()); }
            if unit.as_str() != "ton" { ton.set((kg_f32 / TON).to_string()); }
            if unit.as_str() != "monme" { monme.set((kg_f32 / MONME).to_string()); }
            if unit.as_str() != "ryo" { ryo.set((kg_f32 / RYO).to_string()); }
            if unit.as_str() != "kin" { kin.set((kg_f32 / KIN).to_string()); }
            if unit.as_str() != "kan" { kan.set((kg_f32 / KAN).to_string()); }
        })
    };
    html! {
        <>
        <Header />
        <BreadCrumb />
        <main class="container mt-2">
        <Title title={url::unit_mass(DataMode::Name(lang))} lead={url::unit_mass(DataMode::Dscr(lang))} />
        <div class="row justify-content-md-center">
        <div class={class_core("accordion")} id="accordionUnits">
            <div class="accordion-item">
                <h2 class="accordion-header">
                <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseOne" aria-expanded="false" aria-controls="collapseOne">
                    {_si_unit(lang)}
                </button>
                </h2>
                <div id="collapseOne" class="accordion-collapse collapse show">
                    <div class="accordion-body">
                    <table class="table">
                        <tbody>
                            <UnitMassForm id={"kg"} value={kg.clone()} name={_kg(lang)} onchange={onchange.clone()} />
                        </tbody>
                    </table>
                    </div>
                </div>
            </div>
            <div class="accordion-item">
                <h2 class="accordion-header">
                <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
                    {_yard_pound(lang)}
                </button>
                </h2>
                <div id="collapseTwo" class="accordion-collapse collapse show">
                    <div class="accordion-body">
                    <table class="table">
                        <tbody>
                            <UnitMassForm id={"grain"} value={grain.clone()} name={_grain(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"dram"} value={dram.clone()} name={_dram(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"ounce"} value={ounce.clone()} name={_ounce(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"pound"} value={pound.clone()} name={_pound(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"stone"} value={stone.clone()} name={_stone(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"quarter"} value={quarter.clone()} name={_quarter(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"cental"} value={cental.clone()} name={_cental(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"hrw"} value={hrw.clone()} name={_hrw(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"ton"} value={ton.clone()} name={_ton(lang)} onchange={onchange.clone()} />
                        </tbody>
                    </table>
                    </div>
                </div>
            </div>
            <div class="accordion-item">
                <h2 class="accordion-header">
                <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                    {_shaku_kan(lang)}
                </button>
                </h2>
                <div id="collapseThree" class="accordion-collapse collapse">
                    <div class="accordion-body">
                    <table class="table">
                        <tbody>
                            <UnitMassForm id={"monme"} value={monme.clone()} name={_monme(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"ryo"} value={ryo.clone()} name={_ryo(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"kin"} value={kin.clone()} name={_kin(lang)} onchange={onchange.clone()} />
                            <UnitMassForm id={"kan"} value={kan.clone()} name={_kan(lang)} onchange={onchange.clone()} />
                        </tbody>
                    </table>
                    </div>
                </div>
            </div>
        </div>
        </div>
        </main>
        <Footer />
        </>
    }
}

const GRAIN: f32 = POUND / 7000.0;
const DRAM: f32 = OUNCE / 16.0;
const OUNCE: f32 = POUND / 16.0;
const POUND: f32 = 0.45359237;
const STONE: f32 = POUND * 14.0;
const QUARTER: f32 = POUND * 28.0;
const CENTAL: f32 = POUND * 100.0;
const HRW: f32 = POUND * 112.0;
const TON: f32 = POUND * 2240.0;
const MONME: f32 = RYO / 10.0;
const RYO: f32 = KIN / 16.0;
const KIN: f32 = 0.6;
const KAN: f32 = RYO * 100.0;