use yew::prelude::*;
use crate::Header;

#[derive(Properties, PartialEq)]
pub struct UnitLengthFormProps {
    pub id: String,
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(UnitLengthForm)]
pub fn unit_length_form(props: &UnitLengthFormProps) -> Html {
    let oninput = {
        let value = props.value.clone();
        let on_change = props.on_change.clone();
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
        <div class="mb-3">
            <input type="text" value={props.value.clone()} {oninput} class="form-control" id={props.id.clone()} />
        </div>
    }
}

macro_rules! onchange {
    (
        $us0: ident, $us1: ident, $us2: ident,
        $s0: ident, $s1: ident, $s2: ident
    ) => {
        {
            let us0 = $us0.clone();
            let us1 = $us1.clone();
            let us2 = $us2.clone();
            Callback::from(move |value: String| {
                let meter_f32 = match value.parse::<f32>() {
                    Ok(v_f) => v_f * $s0,
                    Err(_) => 0.0,
                };
                us0.set(value);
                us1.set((meter_f32 / $s1).to_string());
                us2.set((meter_f32 / $s2).to_string());
            })
        }
    };
}

#[function_component(UnitLength)]
pub fn unit_length() -> Html {
    let meter = use_state(|| 1.0_f32.to_string());
    let yard = use_state(|| (1.0 / YARD).to_string());
    let shaku = use_state(|| (1.0 / SHAKU).to_string());

    let onchange_meter = onchange!(meter, yard, shaku, METER, YARD, SHAKU);
    let onchange_yard = onchange!(yard, meter, shaku, YARD, METER, SHAKU);
    let onchange_shaku = onchange!(shaku, meter, yard, SHAKU, METER, YARD);

    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <div class="mb-3">
                    <table class="table">
                        <thread>
                            <th scope="col">{"SI単位"}</th>
                        </thread>
                        <tbody>
                            <tr>
                                <th scope="row">{"メートル"}</th>
                                <td>
                                    <UnitLengthForm id={"meter"} value={(*meter).clone()} on_change={onchange_meter} />
                                </td>
                            </tr>
                            <tr>
                                <th scope="row">{"ヤード"}</th>
                                <td>
                                    <UnitLengthForm id={"yard"} value={(*yard).clone()} on_change={onchange_yard} />
                                </td>
                            </tr>
                            <tr>
                                <th scope="row">{"尺"}</th>
                                <td>
                                    <UnitLengthForm id={"shaku"} value={(*shaku).clone()} on_change={onchange_shaku} />
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </main>
        </>
    }
}

const METER: f32 = 1.0;
const YARD: f32 = 0.9144;
const SHAKU: f32 = 10.0 / 33.0;
//const FATHOM: f32