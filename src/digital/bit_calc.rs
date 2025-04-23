use yew::prelude::*;

use crate::set_lang;
use crate::announce::InvalidInput;
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_core;
use crate::router::{Lang, Route};
use crate::title::Title;

set_lang!(_input, "入力", "Input");
set_lang!(_digits, "桁数", "Digits");
set_lang!(_calc, "計算", "Calculate");
set_lang!(_binary, "2進数表記", "Binary Digits");
set_lang!(_bin, "2進数", "Binary");
set_lang!(_oct, "8進数", "Octal");
set_lang!(_dec, "10進数", "Decimal");
set_lang!(_hex, "16進数", "Hexadecimal");
set_lang!(_result, "計算結果", "Results");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InputForm {
    A, B
}

#[derive(Clone)]
struct Inputs {
    pub a: String,
    pub b: String,
    pub a_base: Base,
    pub b_base: Base,
}

impl Inputs {
    fn new() -> Self {
        Self { a: "".to_string(), b: "".to_string(), a_base: Base::Dec, b_base: Base::Dec }
    }
}

#[derive(Clone)]
struct Outputs {
    pub a_bin: String,
    pub b_bin: String,
    pub bin: String,
    pub oct: String,
    pub dec: String,
    pub hex: String,
}

impl Outputs {
    fn new() -> Self {
        Self {
            a_bin: "".to_string(),
            b_bin: "".to_string(),
            bin: "".to_string(), 
            oct: "".to_string(), 
            dec: "".to_string(), 
            hex: "".to_string()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operator {
    NotA,
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    XNOR,
    ShiftLeft,
    ShiftRight,
}

fn make_on_input(
    input_form: InputForm,
    inputs: UseStateHandle<Inputs>,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
        let mut new_inputs = (*inputs).clone();
        match input_form {
            InputForm::A => { new_inputs.a = input.value(); }
            InputForm::B => { new_inputs.b = input.value(); }
        }
        inputs.set(new_inputs);
    })
}

fn make_on_base_change(
    input_form: InputForm,
    inputs: UseStateHandle<Inputs>
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
        let mut new_inputs = (*inputs).clone();
        let base = match input.value().as_str() {
            "Bin" => Base::Bin,
            "Oct" => Base::Oct,
            "Hex" => Base::Hex,
            _ => Base::Dec,
        };
        match input_form {
            InputForm::A => { new_inputs.a_base = base; }
            InputForm::B => { new_inputs.b_base = base; }
        }
        inputs.set(new_inputs);
    })
}

macro_rules! make_on_submit {
    ($f:ident, $t:ty) => {
        fn $f(
            operator: Operator,
            inputs: UseStateHandle<Inputs>,
            outputs: UseStateHandle<Outputs>,
        ) -> Callback<MouseEvent> {
            Callback::from(move |_: MouseEvent| {
                if let(Ok(a), Ok(b)) = (
                    <$t>::from_str_radix(&inputs.a, inputs.a_base as u32),
                    <$t>::from_str_radix(&inputs.b, inputs.b_base as u32),
                ) {
                    if let Some(result) = match operator {
                        Operator::NotA => Some(!a),
                        Operator::AND => Some(a & b),
                        Operator::OR => Some(a | b),
                        Operator::NAND => Some(!(a & b)),
                        Operator::NOR => Some(!(a | b)),
                        Operator::XOR => Some(a ^ b),
                        Operator::XNOR => Some(!(a ^ b)),
                        Operator::ShiftLeft => match u32::try_from(b) {
                            Ok(b32) => a.checked_shl(b32),
                            Err(_) => None,
                        }
                        Operator::ShiftRight => match u32::try_from(b) {
                            Ok(b32) => a.checked_shr(b32),
                            Err(_) => None,
                        }
                    } {
                        let max_len = a.max(b).next_power_of_two().ilog2() as usize;
                        outputs.set(
                            Outputs {
                                a_bin: format!("{:0width$b}", a, width = max_len),
                                b_bin: format!("{:0width$b}", b, width = max_len),
                                bin: format!("{:b}", result),
                                oct: format!("{:o}", result),
                                dec: format!("{}", result),
                                hex: format!("{:X}", result),
                            }
                        );
                    } else {
                        outputs.set(Outputs::new());
                    }
                }
            })
        }
    };
}

make_on_submit!(make_on_submit_u8, u8);
make_on_submit!(make_on_submit_u16, u16);
make_on_submit!(make_on_submit_u32, u32);
make_on_submit!(make_on_submit_u64, u64);
make_on_submit!(make_on_submit_u128, u128);

#[derive(Clone, Copy, PartialEq, Eq)]
enum UType {
    U8,
    U16,
    U32,
    U64,
    U128
}

fn switch_on_submit(
    utype: UType, 
    operator: Operator,
    inputs: UseStateHandle<Inputs>,
    outputs: UseStateHandle<Outputs>
) -> Callback<MouseEvent> {
    match utype {
        UType::U8 => make_on_submit_u8(operator, inputs, outputs),
        UType::U16 => make_on_submit_u16(operator, inputs, outputs),
        UType::U32 => make_on_submit_u32(operator, inputs, outputs),
        UType::U64 => make_on_submit_u64(operator, inputs, outputs),
        UType::U128 => make_on_submit_u128(operator, inputs, outputs),
    }
}

#[function_component(DigitalBitCalc)]
pub fn digital_bit_calc(props: &HomeProps) -> Html {
    let lang = props.lang;
    let inputs = use_state(Inputs::new);
    let utype = use_state(|| UType::U32);
    let outputs = use_state(Outputs::new);

    let on_utype_change = {
        let utype = utype.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            match input.value().as_str() {
                "u8" => { utype.set(UType::U8); },
                "u16" => { utype.set(UType::U16); },
                "u64" => { utype.set(UType::U64); },
                "u128" => { utype.set(UType::U128); },
                _ => { utype.set(UType::U32); },
            }
        })
    };

    html! {
        <>
        <Header route={Route::DigitalBitCalc} {lang} />
        <BreadCrumb route={Route::DigitalBitCalc} {lang} />
        <main class="container mt-2">
            <Title route={Route::DigitalBitCalc} {lang} />
            <InvalidInput {lang} />
            <div class="row justify-content-md-center">
            <div class={class_core("")}>
            <div class="table-responsive">
            <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="4">{_input(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row">{"A"}</th>
                    <td colspan="2">
                        <input type="text" class="form-control text-end" value={inputs.a.clone()} oninput={make_on_input(InputForm::A, inputs.clone())} />
                    </td>
                    <td>
                        <select class="form-select w-auto" onchange={make_on_base_change(InputForm::A, inputs.clone())}>
                            <option value="Bin">{_bin(lang)}</option>
                            <option value="Oct">{_oct(lang)}</option>
                            <option value="Dec" selected=true>{_dec(lang)}</option>
                            <option value="Hex">{_hex(lang)}</option>
                        </select>
                    </td>
                </tr>
                <tr>
                    <th scope="row">{"B"}</th>
                    <td colspan="2">
                        <input type="text" class="form-control text-end" value={inputs.b.clone()} oninput={make_on_input(InputForm::B, inputs.clone())} />
                    </td>
                    <td>
                        <select class="form-select w-auto" onchange={make_on_base_change(InputForm::B, inputs.clone())}>
                            <option value="Bin">{_bin(lang)}</option>
                            <option value="Oct">{_oct(lang)}</option>
                            <option value="Dec" selected=true>{_dec(lang)}</option>
                            <option value="Hex">{_hex(lang)}</option>
                        </select>
                    </td>
                </tr>

                <tr>
                    <th scope="row" rowspan="2">{_digits(lang)}</th>
                    <td>
                        <div class="form-check form-check-inline">
                            <input class="form-check-input" type="radio" name="utype" value="u8" checked={*utype == UType::U8} onchange={on_utype_change.clone()} />
                            <label class="form-check-label">{"8 bit"}</label>
                        </div>
                    </td>
                    <td>
                        <div class="form-check form-check-inline">
                            <input class="form-check-input" type="radio" name="utype" value="u16" checked={*utype == UType::U16} onchange={on_utype_change.clone()} />
                            <label class="form-check-label">{"16 bit"}</label>
                        </div>
                    </td>
                    <td>
                        <div class="form-check form-check-inline">
                            <input class="form-check-input" type="radio" name="utype" value="u32" checked={*utype == UType::U32} onchange={on_utype_change.clone()} />
                            <label class="form-check-label">{"32 bit"}</label>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="form-check form-check-inline">
                            <input class="form-check-input" type="radio" name="utype" value="u64" checked={*utype == UType::U64} onchange={on_utype_change.clone()} />
                            <label class="form-check-label">{"64 bit"}</label>
                        </div>
                    </td>
                    <td>
                        <div class="form-check form-check-inline">
                            <input class="form-check-input" type="radio" name="utype" value="u128" checked={*utype == UType::U128} onchange={on_utype_change.clone()} />
                            <label class="form-check-label">{"128 bit"}</label>
                        </div>
                    </td>
                </tr>
                <tr>
                    <th scope="row" style="width: 19%" rowspan="3">{_calc(lang)}</th>
                    <td style="width: 27%">
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::NotA, inputs.clone(), outputs.clone())}>{"(NOT) A"}</button>
                        </div>
                    </td>
                    <td style="width: 27%">
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::AND, inputs.clone(), outputs.clone())}>{"A (AND) B"}</button>
                        </div>
                    </td>
                    <td style="width: 27%">
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::OR, inputs.clone(), outputs.clone())}>{"A (OR) B"}</button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::NAND, inputs.clone(), outputs.clone())}>{"A (NAND) B"}</button>
                        </div>
                    </td>
                    <td>
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::NOR, inputs.clone(), outputs.clone())}>{"A (NOR) B"}</button>
                        </div>
                    </td>
                    <td>
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::XOR, inputs.clone(), outputs.clone())}>{"A (XOR) B"}</button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::XNOR, inputs.clone(), outputs.clone())}>{"A (XNOR) B"}</button>
                        </div>
                    </td>
                    <td>
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::ShiftLeft, inputs.clone(), outputs.clone())}>{"A << B"}</button>
                        </div>
                    </td>
                    <td>
                        <div class="d-grid gap-2">
                        <button type="submit" class="btn btn-primary text-nowrap" onclick={switch_on_submit(*utype, Operator::ShiftRight, inputs.clone(), outputs.clone())}>{"A >> B"}</button>
                        </div>
                    </td>
                </tr>
            </tbody>
            </table>

            <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="2">{_binary(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row" style="width: 19%">{"A"}</th>
                    <td style="width: 81%">
                        <input type="text" value={outputs.a_bin.clone()} readonly=true class="form-control text-end" />
                    </td>
                </tr>
                <tr>
                    <th scope="row">{"B"}</th>
                    <td>
                        <input type="text" value={outputs.b_bin.clone()} readonly=true class="form-control text-end" />
                    </td>
                </tr>
            </tbody>
            </table>
            
            <table class="table align-middle">
            <thead>
                <tr><th scope="col" colspan="2">{_result(lang)}</th></tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row" style="width: 19%">{_bin(lang)}</th>
                    <td style="width: 81%">
                        <input type="text" value={outputs.bin.clone()} readonly=true class="form-control text-end" />
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_oct(lang)}</th>
                    <td>
                        <input type="text" value={outputs.oct.clone()} readonly=true class="form-control text-end" />
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_dec(lang)}</th>
                    <td>
                        <input type="text" value={outputs.dec.clone()} readonly=true class="form-control text-end" />
                    </td>
                </tr>
                <tr>
                    <th scope="row">{_hex(lang)}</th>
                    <td>
                        <input type="text" value={outputs.hex.clone()} readonly=true class="form-control text-end" />
                    </td>
                </tr>
            </tbody>
            </table>

            </div>
            </div>
            </div>
            <Footer {lang} />
        </main>
        </>
    }
}