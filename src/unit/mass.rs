pub mod ja;
pub mod en;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::parse_state;

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
            <th scope="row">{props.name.clone()}</th>
            <td>
                <input type="number" step="0.1" value={(*props.value).clone()} {onchange} class="form-control" id={props.id.clone()} />
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct UnitMassBaseProps {
    pub title: String,
    pub lead: String,
    pub si_unit: String, pub yard_pound: String, pub shaku_kan: String,
    pub kg: String,
    pub grain: String, pub dram: String, pub ounce: String, pub pound: String, 
    pub stone: String, pub quarter: String, pub cental: String, pub hrw: String, pub ton: String,
    pub monme: String, pub ryo: String, pub kin: String, pub kan: String,
}

#[function_component(UnitMassBase)]
pub fn unit_mass(props: &UnitMassBaseProps) -> Html {
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
        <main class="container-fluid mt-2">
        <div class="container">
            <h1 class="mt-3">{&props.title}</h1>
            <p class="lead">{&props.lead}</p>
        </div>
        <div class="accordion" id="accordionUnits">
            <div class="accordion-item">
                <h2 class="accordion-header">
                <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseOne" aria-expanded="false" aria-controls="collapseOne">
                    {&props.si_unit}
                </button>
                </h2>
                <div id="collapseOne" class="accordion-collapse collapse show">
                    <div class="accordion-body">
                    <table class="table">
                        <tbody>
                            <UnitMassForm id={"kg"} value={kg.clone()} name={props.kg.clone()} onchange={onchange.clone()} />
                        </tbody>
                    </table>
                    </div>
                </div>
            </div>
            <div class="accordion-item">
                <h2 class="accordion-header">
                <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
                    {&props.yard_pound}
                </button>
                </h2>
                <div id="collapseTwo" class="accordion-collapse collapse show">
                    <div class="accordion-body">
                    <table class="table">
                        <tbody>
                            <UnitMassForm id={"grain"} value={grain.clone()} name={props.grain.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"dram"} value={dram.clone()} name={props.dram.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"ounce"} value={ounce.clone()} name={props.ounce.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"pound"} value={pound.clone()} name={props.pound.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"stone"} value={stone.clone()} name={props.stone.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"quarter"} value={quarter.clone()} name={props.quarter.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"cental"} value={cental.clone()} name={props.cental.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"hrw"} value={hrw.clone()} name={props.hrw.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"ton"} value={ton.clone()} name={props.ton.clone()} onchange={onchange.clone()} />
                        </tbody>
                    </table>
                    </div>
                </div>
            </div>
            <div class="accordion-item">
                <h2 class="accordion-header">
                <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                    {&props.shaku_kan}
                </button>
                </h2>
                <div id="collapseThree" class="accordion-collapse collapse">
                    <div class="accordion-body">
                    <table class="table">
                        <tbody>
                            <UnitMassForm id={"monme"} value={monme.clone()} name={props.monme.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"ryo"} value={ryo.clone()} name={props.ryo.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"kin"} value={kin.clone()} name={props.kin.clone()} onchange={onchange.clone()} />
                            <UnitMassForm id={"kan"} value={kan.clone()} name={props.kan.clone()} onchange={onchange.clone()} />
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
        </div>
        </main>
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