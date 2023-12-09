pub mod ja;
pub mod en;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnitMassFormProps {
    pub id: String,
    pub value: String,
    pub name: String,
    pub onchange: Callback<String>,
}

#[function_component(UnitMassForm)]
pub fn unit_mass_form(props: &UnitMassFormProps) -> Html {
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
macro_rules! unit_mass {
    (
        $title: expr,
        $si_unit: expr, $yard_pound: expr, $shaku_kan: expr,
        $kg: expr,
        $grain: expr, $dram: expr, $ounce: expr, $pound: expr, $stone: expr, 
        $quarter: expr, $cental: expr, $hrw: expr, $ton: expr, 
        $monme: expr, $ryo: expr, $kin: expr, $kan: expr
    ) => {
        macro_rules! onchange {
            (
                $us0: ident, $us1: ident, $us2: ident, $us3: ident, $us4: ident, $us5: ident, $us6: ident,
                $us7: ident, $us8: ident, $us9: ident, $us10: ident, $us11: ident, $us12: ident, $us13: ident,
                $s0: ident, $s1: ident, $s2: ident, $s3: ident, $s4: ident, $s5: ident, $s6: ident, 
                $s7: ident, $s8: ident, $s9: ident, $s10: ident, $s11: ident, $s12: ident, $s13: ident
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
                    Callback::from(move |value: String| {
                        let kg_f32 = match value.parse::<f32>() {
                            Ok(v_f) => v_f * $s0,
                            Err(_) => 0.0,
                        };
                        us0.set(value);
                        us1.set((kg_f32 / $s1).to_string());
                        us2.set((kg_f32 / $s2).to_string());
                        us3.set((kg_f32 / $s3).to_string());
                        us4.set((kg_f32 / $s4).to_string());
                        us5.set((kg_f32 / $s5).to_string());
                        us6.set((kg_f32 / $s6).to_string());
                        us7.set((kg_f32 / $s7).to_string());
                        us8.set((kg_f32 / $s8).to_string());
                        us9.set((kg_f32 / $s9).to_string());
                        us10.set((kg_f32 / $s10).to_string());
                        us11.set((kg_f32 / $s11).to_string());
                        us12.set((kg_f32 / $s12).to_string());
                        us13.set((kg_f32 / $s13).to_string());
                    })
                }
            };
        }
        
        #[function_component(UnitMass)]
        pub fn unit_mass() -> Html {
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
        
            let onchange_kg = onchange!(
                kg, grain, dram, ounce, pound, stone, quarter, cental, hrw, ton, monme, ryo, kin, kan, 
                KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN, KAN);
            let onchange_grain = onchange!(
                grain, kg, dram, ounce, pound, stone, quarter, cental, hrw, ton, monme, ryo, kin, kan, 
                GRAIN, KG, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN, KAN);
            let onchange_dram = onchange!(
                dram, kg, grain, ounce, pound, stone, quarter, cental, hrw, ton, monme, ryo, kin, kan, 
                DRAM, KG, GRAIN, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN, KAN);
            let onchange_ounce = onchange!(
                ounce, kg, grain, dram, pound, stone, quarter, cental, hrw, ton, monme, ryo, kin, kan, 
                OUNCE, KG, GRAIN, DRAM, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN, KAN);
            let onchange_pound = onchange!(
                pound, kg, grain, dram, ounce, stone, quarter, cental, hrw, ton, monme, ryo, kin, kan, 
                POUND, KG, GRAIN, DRAM, OUNCE, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN, KAN);
            let onchange_stone = onchange!(
                stone, kg, grain, dram, ounce, pound, quarter, cental, hrw, ton, monme, ryo, kin, kan, 
                STONE, KG, GRAIN, DRAM, OUNCE, POUND, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN, KAN);
            let onchange_quarter = onchange!(
                quarter, kg, grain, dram, ounce, pound, stone, cental, hrw, ton, monme, ryo, kin, kan, 
                QUARTER, KG, GRAIN, DRAM, OUNCE, POUND, STONE, CENTAL, HRW, TON, MONME, RYO, KIN, KAN);
            let onchange_cental = onchange!(
                cental, kg, grain, dram, ounce, pound, stone, quarter, hrw, ton, monme, ryo, kin, kan, 
                CENTAL, KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, HRW, TON, MONME, RYO, KIN, KAN);
            let onchange_hrw = onchange!(
                hrw, kg, grain, dram, ounce, pound, stone, quarter, cental, ton, monme, ryo, kin, kan, 
                HRW, KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, TON, MONME, RYO, KIN, KAN);
            let onchange_ton = onchange!(
                ton, kg, grain, dram, ounce, pound, stone, quarter, cental, hrw, monme, ryo, kin, kan, 
                TON, KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, MONME, RYO, KIN, KAN);
            let onchange_monme = onchange!(
                monme, kg, grain, dram, ounce, pound, stone, quarter, cental, hrw, ton, ryo, kin, kan, 
                MONME, KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, RYO, KIN, KAN);
            let onchange_ryo = onchange!(
                ryo, kg, grain, dram, ounce, pound, stone, quarter, cental, hrw, ton, monme, kin, kan, 
                RYO, KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, KIN, KAN);
            let onchange_kin = onchange!(
                kin, kg, grain, dram, ounce, pound, stone, quarter, cental, hrw, ton, monme, ryo, kan, 
                KIN, KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KAN);
            let onchange_kan = onchange!(
                kan, kg, grain, dram, ounce, pound, stone, quarter, cental, hrw, ton, monme, ryo, kin, 
                KAN, KG, GRAIN, DRAM, OUNCE, POUND, STONE, QUARTER, CENTAL, HRW, TON, MONME, RYO, KIN);
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
                                        <UnitMassForm id={"kg"} value={(*kg).clone()} name={$kg} onchange={onchange_kg} />
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
                                        <UnitMassForm id={"grain"} value={(*grain).clone()} name={$grain} onchange={onchange_grain} />
                                        <UnitMassForm id={"dram"} value={(*dram).clone()} name={$dram} onchange={onchange_dram} />
                                        <UnitMassForm id={"ounce"} value={(*ounce).clone()} name={$ounce} onchange={onchange_ounce} />
                                        <UnitMassForm id={"pound"} value={(*pound).clone()} name={$pound} onchange={onchange_pound} />
                                        <UnitMassForm id={"stone"} value={(*stone).clone()} name={$stone} onchange={onchange_stone} />
                                        <UnitMassForm id={"quarter"} value={(*quarter).clone()} name={$quarter} onchange={onchange_quarter} />
                                        <UnitMassForm id={"cental"} value={(*cental).clone()} name={$cental} onchange={onchange_cental} />
                                        <UnitMassForm id={"hrw"} value={(*hrw).clone()} name={$hrw} onchange={onchange_hrw} />
                                        <UnitMassForm id={"ton"} value={(*ton).clone()} name={$ton} onchange={onchange_ton} />
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
                                        <UnitMassForm id={"monme"} value={(*monme).clone()} name={$monme} onchange={onchange_monme} />
                                        <UnitMassForm id={"ryo"} value={(*ryo).clone()} name={$ryo} onchange={onchange_ryo} />
                                        <UnitMassForm id={"kin"} value={(*kin).clone()} name={$kin} onchange={onchange_kin} />
                                        <UnitMassForm id={"kan"} value={(*kan).clone()} name={$kan} onchange={onchange_kan} />
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

const KG: f32 = 1.0;
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