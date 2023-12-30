use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::announce::InvalidInput;
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::layout::class_core;
use crate::router::parse_query;
use crate::url::{self, DataMode, Lang};
use crate::{parse_state, set_lang};
use crate::title::Title;

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
set_lang!(_maritime, "海事系", "Maritime System");
set_lang!(_meter, "メートル (m)", "Metre (m)");
set_lang!(_inch, "インチ (in)", "Inch (in)");
set_lang!(_feet, "フィート (ft)", "Feet (ft)");
set_lang!(_yard, "ヤード (yd)", "Yard (yd)");
set_lang!(_chain, "チェーン", "Chain");
set_lang!(_furlong, "ハロン (fur)", "Furlong (fur)");
set_lang!(_mile, "マイル (mi)", "Mile (mi)");
set_lang!(_sun, "寸", "寸 -Sun-");
set_lang!(_shaku, "尺", "尺 -Shaku-");
set_lang!(_ken, "間・歩・尋", "間 -Ken-  歩-Bu-  尋-Hiro-");
set_lang!(_cho, "丈", "丈 -Jo-");
set_lang!(_jo, "町", "町 -Cho-");
set_lang!(_ri, "里", "里 -Ri-");
set_lang!(_kairi, "海里", "Nautical mile");
set_lang!(_fathom, "ファゾム", "Fathom");

#[function_component(UnitLength)]
pub fn unit_base() -> Html {
    let lang = match parse_query(use_location().unwrap().query_str()).1 {
        Some(Lang::Ja) => Lang::Ja, _ => Lang::En
    };
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
        <>
        <Header />
        <BreadCrumb />
        <main class="container mt-2">
        <Title title={url::unit_length(DataMode::Name(lang))} lead={url::unit_length(DataMode::Dscr(lang))} />
        <InvalidInput />
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
                            <UnitLengthForm id={"meter"} value={meter.clone()} name={_meter(lang)} onchange={onchange.clone()} />
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
                                <UnitLengthForm id={"inch"} value={inch.clone()} name={_inch(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"feet"} value={feet.clone()} name={_feet(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"yard"} value={yard.clone()} name={_yard(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"chain"} value={chain.clone()} name={_chain(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"furlong"} value={furlong.clone()} name={_furlong(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"mile"} value={mile.clone()} name={_mile(lang)} onchange={onchange.clone()} />
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
                                <UnitLengthForm id={"sun"} value={sun.clone()} name={_sun(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"shaku"} value={shaku.clone()} name={_shaku(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"ken"} value={ken.clone()} name={_ken(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"jo"} value={jo.clone()} name={_jo(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"cho"} value={cho.clone()} name={_cho(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"ri"} value={ri.clone()} name={_ri(lang)} onchange={onchange.clone()} />
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
            <div class="accordion-item">
                <h2 class="accordion-header">
                <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseFour" aria-expanded="false" aria-controls="collapseFour">
                    {_maritime(lang)}
                </button>
                </h2>
                <div id="collapseFour" class="accordion-collapse collapse">
                    <div class="accordion-body">
                        <table class="table">
                            <tbody>
                                <UnitLengthForm id={"kairi"} value={kairi.clone()} name={_kairi(lang)} onchange={onchange.clone()} />
                                <UnitLengthForm id={"fathom"} value={fathom.clone()} name={_fathom(lang)} onchange={onchange.clone()} />
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

const INCH: f32 = FEET / 12.0;
pub const FEET: f32 = 0.3048;
pub const YARD: f32 = FEET * 3.0;
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