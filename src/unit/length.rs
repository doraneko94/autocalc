pub mod ja;
pub mod en;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnitLengthFormProps {
    pub id: String,
    pub value: String,
    pub name: String,
    pub onchange: Callback<String>,
}

#[function_component(UnitLengthForm)]
pub fn unit_length_form(props: &UnitLengthFormProps) -> Html {
    let oninput = {
        let value = props.value.clone();
        let on_change = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            let v_str = e.data();

            let value = match v_str {
                Some(v) => { value.clone() + &v }
                None => { "".to_string() }
            };
            e.prevent_default();
            on_change.emit(value);
        })
    };
    html! {
        <tr>
            <th scope="row">{props.name.clone()}</th>
            <td>
                <input type="text" value={props.value.clone()} {oninput} class="form-control" id={props.id.clone()} />
            </td>
        </tr>
    }
}

#[macro_export]
macro_rules! unit_length {
    (
        $title: expr,
        $si_unit: expr, $yard_pound: expr, $shaku_kan: expr, $maritime: expr,
        $meter: expr,
        $inch: expr, $feet: expr, $yard: expr, $chain: expr, $furlong: expr, $mile: expr,
        $sun: expr, $shaku: expr, $ken: expr, $jo: expr, $cho: expr, $ri: expr,
        $kairi: expr, $fathom: expr 
    ) => {
        macro_rules! onchange {
            (
                $us0: ident, $us1: ident, $us2: ident, $us3: ident, $us4: ident, $us5: ident, $us6: ident,
                $us7: ident, $us8: ident, $us9: ident, $us10: ident, $us11: ident, $us12: ident, $us13: ident, $us14: ident,
                $s0: ident, $s1: ident, $s2: ident, $s3: ident, $s4: ident, $s5: ident, $s6: ident, 
                $s7: ident, $s8: ident, $s9: ident, $s10: ident, $s11: ident, $s12: ident, $s13: ident, $s14: ident
            ) => {
                {
                    let us0 = $us0.clone();
                    let us1 = $us1.clone();
                    let us2 = $us2.clone();
                    let us3 = $us3.clone();
                    let us4 = $us4.clone();
                    let us5 = $us5.clone();
                    let us6 = $us6.clone();
                    let us7 = $us7.clone();
                    let us8 = $us8.clone();
                    let us9 = $us9.clone();
                    let us10 = $us10.clone();
                    let us11 = $us11.clone();
                    let us12 = $us12.clone();
                    let us13 = $us13.clone();
                    let us14 = $us14.clone();
                    Callback::from(move |value: String| {
                        let meter_f32 = match value.parse::<f32>() {
                            Ok(v_f) => v_f * $s0,
                            Err(_) => 0.0,
                        };
                        us0.set(value);
                        us1.set((meter_f32 / $s1).to_string());
                        us2.set((meter_f32 / $s2).to_string());
                        us3.set((meter_f32 / $s3).to_string());
                        us4.set((meter_f32 / $s4).to_string());
                        us5.set((meter_f32 / $s5).to_string());
                        us6.set((meter_f32 / $s6).to_string());
                        us7.set((meter_f32 / $s7).to_string());
                        us8.set((meter_f32 / $s8).to_string());
                        us9.set((meter_f32 / $s9).to_string());
                        us10.set((meter_f32 / $s10).to_string());
                        us11.set((meter_f32 / $s11).to_string());
                        us12.set((meter_f32 / $s12).to_string());
                        us13.set((meter_f32 / $s13).to_string());
                        us14.set((meter_f32 / $s14).to_string());
                    })
                }
            };
        }
        
        #[function_component(UnitLength)]
        pub fn unit_length() -> Html {
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
        
            let onchange_meter = onchange!(
                meter, inch, feet, yard, chain, furlong, mile, sun, shaku, ken, jo, cho, ri, kairi, fathom, 
                METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_inch = onchange!(
                inch, meter, feet, yard, chain, furlong, mile, sun, shaku, ken, jo, cho, ri, kairi, fathom, 
                INCH, METER, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_feet = onchange!(
                feet, meter, inch, yard, chain, furlong, mile, sun, shaku, ken, jo, cho, ri, kairi, fathom, 
                FEET, METER, INCH, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_yard = onchange!(
                yard, meter, inch, feet, chain, furlong, mile, sun, shaku, ken, jo, cho, ri, kairi, fathom, 
                YARD, METER, INCH, FEET, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_chain = onchange!(
                chain, meter, inch, feet, yard, furlong, mile, sun, shaku, ken, jo, cho, ri, kairi, fathom, 
                CHAIN, METER, INCH, FEET, YARD, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_furlong = onchange!(
                furlong, meter, inch, feet, yard, chain, mile, sun, shaku, ken, jo, cho, ri, kairi, fathom, 
                FURLONG, METER, INCH, FEET, YARD, CHAIN, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_mile = onchange!(
                mile, meter, inch, feet, yard, chain, furlong, sun, shaku, ken, jo, cho, ri, kairi, fathom, 
                MILE, METER, INCH, FEET, YARD, CHAIN, FURLONG, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_sun = onchange!(
                sun, meter, inch, feet, yard, chain, furlong, mile, shaku, ken, jo, cho, ri, kairi, fathom, 
                SUN, METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SHAKU, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_shaku = onchange!(
                shaku, meter, inch, feet, yard, chain, furlong, mile, sun, ken, jo, cho, ri, kairi, fathom, 
                SHAKU, METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, KEN, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_ken = onchange!(
                ken, meter, inch, feet, yard, chain, furlong, mile, sun, shaku, jo, cho, ri, kairi, fathom, 
                KEN, METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, JO, CHO, RI, KAIRI, FATHOM);
            let onchange_jo = onchange!(
                jo, meter, inch, feet, yard, chain, furlong, mile, sun, shaku, ken, cho, ri, kairi, fathom, 
                JO, METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, CHO, RI, KAIRI, FATHOM);
            let onchange_cho = onchange!(
                cho, meter, inch, feet, yard, chain, furlong, mile, sun, shaku, ken, jo, ri, kairi, fathom, 
                CHO, METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, RI, KAIRI, FATHOM);
            let onchange_ri = onchange!(
                ri, meter, inch, feet, yard, chain, furlong, mile, sun, shaku, ken, jo, cho, kairi, fathom, 
                RI, METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, KAIRI, FATHOM);
            let onchange_kairi = onchange!(
                kairi, meter, inch, feet, yard, chain, furlong, mile, sun, shaku, ken, jo, cho, ri, fathom, 
                KAIRI, METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, FATHOM);
            let onchange_fathom = onchange!(
                fathom, meter, inch, feet, yard, chain, furlong, mile, sun, shaku, ken, jo, cho, ri, kairi, 
                FATHOM, METER, INCH, FEET, YARD, CHAIN, FURLONG, MILE, SUN, SHAKU, KEN, JO, CHO, RI, KAIRI);
            html! {
                <>
                    <Header />
                    <h2>{$title}</h2>
                    <main class="container-fluid mt-2">
                    <div class="accordion" id="accordionUnits">
                        <div class="accordion-item">
                            <h2 class="accordion-header">
                            <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseOne" aria-expanded="false" aria-controls="collapseOne">
                                {$si_unit}
                            </button>
                            </h2>
                            <div id="collapseOne" class="accordion-collapse collapse show">
                            <div class="accordion-body">
                                <table class="table">
                                    <tbody>
                                        <UnitLengthForm id={"meter"} value={(*meter).clone()} name={$meter} onchange={onchange_meter} />
                                    </tbody>
                                </table>
                            </div>
                            </div>
                        </div>
                        <div class="accordion-item">
                            <h2 class="accordion-header">
                            <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
                                {$yard_pound}
                            </button>
                            </h2>
                            <div id="collapseTwo" class="accordion-collapse collapse show">
                            <div class="accordion-body">
                                <table class="table">
                                    <tbody>
                                        <UnitLengthForm id={"inch"} value={(*inch).clone()} name={$inch} onchange={onchange_inch} />
                                        <UnitLengthForm id={"feet"} value={(*feet).clone()} name={$feet} onchange={onchange_feet} />
                                        <UnitLengthForm id={"yard"} value={(*yard).clone()} name={$yard} onchange={onchange_yard} />
                                        <UnitLengthForm id={"chain"} value={(*chain).clone()} name={$chain} onchange={onchange_chain} />
                                        <UnitLengthForm id={"furlong"} value={(*furlong).clone()} name={$furlong} onchange={onchange_furlong} />
                                        <UnitLengthForm id={"mile"} value={(*mile).clone()} name={$mile} onchange={onchange_mile} />
                                    </tbody>
                                </table>
                            </div>
                            </div>
                        </div>
                        <div class="accordion-item">
                            <h2 class="accordion-header">
                            <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                                {$shaku_kan}
                            </button>
                            </h2>
                            <div id="collapseThree" class="accordion-collapse collapse">
                            <div class="accordion-body">
                                <table class="table">
                                    <tbody>
                                        <UnitLengthForm id={"sun"} value={(*sun).clone()} name={$sun} onchange={onchange_sun} />
                                        <UnitLengthForm id={"shaku"} value={(*shaku).clone()} name={$shaku} onchange={onchange_shaku} />
                                        <UnitLengthForm id={"ken"} value={(*ken).clone()} name={$ken} onchange={onchange_ken} />
                                        <UnitLengthForm id={"jo"} value={(*jo).clone()} name={$jo} onchange={onchange_jo} />
                                        <UnitLengthForm id={"cho"} value={(*cho).clone()} name={$cho} onchange={onchange_cho} />
                                        <UnitLengthForm id={"ri"} value={(*ri).clone()} name={$ri} onchange={onchange_ri} />
                                    </tbody>
                                </table>
                            </div>
                        </div>
                        </div>
                        <div class="accordion-item">
                            <h2 class="accordion-header">
                            <button class="accordion-button collapsed btn-outline-dark" type="button" data-bs-toggle="collapse" data-bs-target="#collapseFour" aria-expanded="false" aria-controls="collapseFour">
                                {$maritime}
                            </button>
                            </h2>
                            <div id="collapseFour" class="accordion-collapse collapse">
                            <div class="accordion-body">
                                <table class="table">
                                    <tbody>
                                        <UnitLengthForm id={"kairi"} value={(*kairi).clone()} name={$kairi} onchange={onchange_kairi} />
                                        <UnitLengthForm id={"fathom"} value={(*fathom).clone()} name={$fathom} onchange={onchange_fathom} />
                                    </tbody>
                                </table>
                            </div>
                        </div>
                        </div>
                    </div>
                    </main>
                </>
            }
        }
    };
}

const METER: f32 = 1.0;
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