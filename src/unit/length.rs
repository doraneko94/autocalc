pub mod ja;
pub mod en;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::parse_state;

#[derive(Properties, PartialEq)]
pub struct UnitLengthFormProps {
    pub id: String,
    pub value: UseStateHandle<String>,
    pub name: String,
    pub onchange: Callback<(String, String)>,
}

#[function_component(UnitLengthForm)]
pub fn unit_length_form(props: &UnitLengthFormProps) -> Html {
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
pub struct UnitLengthBaseProps {
    pub title: String,
    pub lead: String,
    pub si_unit: String, pub yard_pound: String, pub shaku_kan: String, pub maritime: String,
    pub meter: String, 
    pub inch: String, pub feet: String, pub yard: String, pub chain: String, pub furlong: String, pub mile: String,
    pub sun: String, pub shaku: String, pub ken: String, pub cho: String, pub jo: String, pub ri: String,
    pub kairi: String, pub fathom: String
}

#[function_component(UnitLengthBase)]
pub fn unit_length_base(props: &UnitLengthBaseProps) -> Html {
    let meter = use_state(|| 1.0_f32.to_string());
    let inch = use_state(|| (1.0 / INCH).to_string());
    let feet = use_state(|| (1.0 / FEET).to_string());
    let yard = use_state(|| (1.0 / YARD).to_string());
    let chain = use_state(|| (1.0 / CHAIN).to_string());
    let furlong = use_state(|| (1.0 / FURLONG).to_string());
    let mile = use_state(|| (1.0 / MILE).to_string());
    let sun = use_state(|| (1.0 / SUN).to_string());
    let shaku = use_state(|| (1.0 / SHAKU).to_string());
    let ken = use_state(|| (1.0 / KEN).to_string());
    let jo = use_state(|| (1.0 / JO).to_string());
    let cho = use_state(|| (1.0 / CHO).to_string());
    let ri = use_state(|| (1.0 / RI).to_string());
    let kairi = use_state(|| (1.0 / KAIRI).to_string());
    let fathom = use_state(|| (1.0 / FATHOM).to_string());
        
    let onchange = {
        let meter = meter.clone();
        let inch = inch.clone();
        let feet = feet.clone();
        let yard = yard.clone();
        let chain = chain.clone();
        let furlong = furlong.clone();
        let mile = mile.clone();
        let sun = sun.clone();
        let shaku = shaku.clone();
        let ken = ken.clone();
        let jo = jo.clone();
        let cho = cho.clone();
        let ri = ri.clone();
        let kairi = kairi.clone();
        let fathom = fathom.clone();

        Callback::from(move |value: (String, String)| {
            let (s, unit) = value;
            let mut meter_f32 = parse_state!(s, f32);
            meter_f32 = match unit.as_str() {
                "meter" => meter_f32,
                "inch" => meter_f32 * INCH,
                "feet" => meter_f32 * FEET,
                "yard" => meter_f32 * YARD,
                "chain" => meter_f32 * CHAIN,
                "furlong" => meter_f32 * FURLONG,
                "mile" => meter_f32 * MILE,
                "sun" => meter_f32 * SUN,
                "shaku" => meter_f32 * SHAKU,
                "ken" => meter_f32 * KEN,
                "jo" => meter_f32 * JO,
                "cho" => meter_f32 * CHO,
                "ri" => meter_f32 * RI,
                "kairi" => meter_f32 * KAIRI,
                "fathom" => meter_f32 * FATHOM,
                _ => 0.0
            };
            if unit.as_str() != "meter" { meter.set(meter_f32.to_string()); }
            if unit.as_str() != "inch" { inch.set((meter_f32 / INCH).to_string()); }
            if unit.as_str() != "feet" { feet.set((meter_f32 / FEET).to_string()); }
            if unit.as_str() != "yard" { yard.set((meter_f32 / YARD).to_string()); }
            if unit.as_str() != "chain" { chain.set((meter_f32 / CHAIN).to_string()); }
            if unit.as_str() != "furlong" { furlong.set((meter_f32 / FURLONG).to_string()); }
            if unit.as_str() != "mile" { mile.set((meter_f32 / MILE).to_string()); }
            if unit.as_str() != "sun" { sun.set((meter_f32 / SUN).to_string()); }
            if unit.as_str() != "shaku" { shaku.set((meter_f32 / SHAKU).to_string()); }
            if unit.as_str() != "ken" { ken.set((meter_f32 / KEN).to_string()); }
            if unit.as_str() != "jo" { jo.set((meter_f32 / JO).to_string()); }
            if unit.as_str() != "cho" { cho.set((meter_f32 / CHO).to_string()); }
            if unit.as_str() != "ri" { ri.set((meter_f32 / RI).to_string()); }
            if unit.as_str() != "kairi" { kairi.set((meter_f32 / KAIRI).to_string()); }
            if unit.as_str() != "fathom" { fathom.set((meter_f32 / FATHOM).to_string()); }
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
                            <UnitLengthForm id={"meter"} value={meter.clone()} name={props.meter.clone()} onchange={onchange.clone()} />
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
                                <UnitLengthForm id={"inch"} value={inch.clone()} name={props.inch.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"feet"} value={feet.clone()} name={props.feet.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"yard"} value={yard.clone()} name={props.yard.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"chain"} value={chain.clone()} name={props.chain.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"furlong"} value={furlong.clone()} name={props.furlong.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"mile"} value={mile.clone()} name={props.mile.clone()} onchange={onchange.clone()} />
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
                                <UnitLengthForm id={"sun"} value={sun.clone()} name={props.sun.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"shaku"} value={shaku.clone()} name={props.shaku.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"ken"} value={ken.clone()} name={props.ken.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"jo"} value={jo.clone()} name={props.jo.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"cho"} value={cho.clone()} name={props.cho.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"ri"} value={ri.clone()} name={props.ri.clone()} onchange={onchange.clone()} />
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
            <div class="accordion-item">
                <h2 class="accordion-header">
                <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseFour" aria-expanded="false" aria-controls="collapseFour">
                    {&props.maritime}
                </button>
                </h2>
                <div id="collapseFour" class="accordion-collapse collapse">
                    <div class="accordion-body">
                        <table class="table">
                            <tbody>
                                <UnitLengthForm id={"kairi"} value={kairi.clone()} name={props.kairi.clone()} onchange={onchange.clone()} />
                                <UnitLengthForm id={"fathom"} value={fathom.clone()} name={props.fathom.clone()} onchange={onchange.clone()} />
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
        </main>
    }
}

const INCH: f32 = FEET / 12.0;
const FEET: f32 = 0.3048;
const YARD: f32 = FEET * 3.0;
const CHAIN: f32 = YARD * 22.0;
const FURLONG: f32 = YARD * 220.0;
const MILE: f32 = YARD * 1760.0;
const SUN: f32 = SHAKU / 10.0;
const SHAKU: f32 = 10.0 / 33.0;
const KEN: f32 = SHAKU * 6.0;
const JO: f32 = SHAKU * 10.0;
const CHO: f32 = KEN * 60.0;
const RI: f32 = CHO * 36.0;
const KAIRI: f32 = 1852.0;
const FATHOM: f32 = YARD * 6.0;