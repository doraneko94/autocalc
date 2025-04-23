use base::{Radix, convert_all};
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ClipboardEvent, HtmlCanvasElement, HtmlInputElement};
use yew::prelude::*;

use crate::announce::InvalidInput;
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_core;
use crate::router::{Lang, Route};
use crate::set_lang;
use crate::title::Title;

set_lang!(_value, "数値", "Value");
set_lang!(_digits, "最低桁数", "Min. Num. of Digits");
set_lang!(_bin, "2進数", "Binary");
set_lang!(_oct, "8進数", "Octal");
set_lang!(_dec, "10進数", "Decimal");
set_lang!(_hex, "16進数", "Hexadecimal");

fn padding_zero(s: &str, n_pad: usize) -> String {
    let s = match s.trim_start_matches("0") {
        "" => "0",
        s => s,
    };
    let size = s.len();
    if size < n_pad {
        format!("{}{}", "0".repeat(n_pad - size), s)
    } else {
        s.to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct NumberState {
    binary: String,
    octal: String,
    decimal: String,
    hexadecimal: String,
    bin_pad: usize,
    oct_pad: usize,
    dec_pad: usize,
    hex_pad: usize,
}

impl NumberState {
    fn new() -> Self {
        Self {
            binary: String::new(),
            octal: String::new(),
            decimal: String::new(),
            hexadecimal: String::new(),
            bin_pad: 1,
            oct_pad: 1,
            dec_pad: 1,
            hex_pad: 1
        }
    }

    fn update_from_value(&mut self, input: &str, radix: Radix) {
        if let Some((s2, s8, s10, s16)) = convert_all(input, radix) {
            self.binary = padding_zero(&s2, self.bin_pad);
            self.octal = padding_zero(&s8, self.oct_pad);
            self.decimal = padding_zero(&s10, self.dec_pad);
            self.hexadecimal = padding_zero(&s16.to_uppercase(), self.hex_pad);
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct GroupedInputProps {
    pub label: String,
    pub value: String,
    pub radix: Radix, 
    pub pad_value: usize,
    pub group: usize,

    pub on_input: Callback<(String, Radix)>,
    pub on_pad_change: Callback<(Radix, usize)>
}

#[function_component(GroupedInput)]
pub fn grouped_input(props: &GroupedInputProps) -> Html {
    let input_ref = use_node_ref();
    let canvas_ref = use_node_ref();
    let local_value = use_state(|| props.value.clone());

    {
        let local_value = local_value.clone();
        let value_from_props = props.value.clone();
        use_effect_with(
            value_from_props,
            move |new_value| {
                if &*local_value != new_value {
                    local_value.set(new_value.clone());
                }
                || ()
            }
        );
    }

    use_effect_with(
        (),
        {
            let input_ref = input_ref.clone();
            let canvas_ref = canvas_ref.clone();
            let group = props.group;
            move |_| {
                let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                let ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                let width = input.client_width();
                let height = input.client_height();
                canvas.set_width(width as u32);
                canvas.set_height(height as u32);

                ctx.set_font("1rem 'Roboto Mono', 'Consolas', 'Menlo', 'Courier New', monospace");
                let char_width = ctx.measure_text("0").unwrap().width();

                let mut i = 0;

                loop {
                    let start_x = width as f64 - (i + 1) as f64 * char_width * group as f64;
                    let end_x = width as f64 - i as f64 * char_width * group as f64;

                    if end_x < 0.0 {
                        break;
                    }

                    ctx.set_fill_style_str(
                        if i % 2 == 0 { "#ffffff" }
                        else { "#eeeeee" }
                    );
                    ctx.fill_rect(start_x, 0.0, end_x - start_x, height as f64);

                    i += 1;
                }

                let listener = EventListener::new(&input, "paste", move |event| {
                    let input: HtmlInputElement = event
                        .target()
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap();

                    let evt = event.dyn_ref::<ClipboardEvent>().unwrap();
                    let mut paste = evt.clipboard_data().unwrap().get_data("text").unwrap();
                    paste.retain(|c| c.is_ascii_hexdigit());

                    let start = input.selection_start().unwrap().unwrap();
                    let end = input.selection_end().unwrap().unwrap();
                    let mut value = input.value();
                    value.replace_range(start as usize..end as usize, &paste);
                    input.set_value(&value);
                    input.set_selection_start(Some(start + paste.len() as u32)).ok();
                    input.set_selection_end(Some(start + paste.len() as u32)).ok();

                    event.prevent_default();
                });

                move || drop(listener)
            }
        }
    );

    html! {
        <tr>
        <th scope="row">{props.label.clone()}</th>
        <td>
        <div class="wrapper">
        <canvas ref={canvas_ref} class="canvas-overlay"></canvas>
        <input
            class="form-control input-field text-end bg-transparent"
            ref={input_ref}
            value={(*local_value).clone()}
            oninput={
                let local_value = local_value.clone();
                let on_input = props.on_input.clone();
                let radix = props.radix.clone();
                let pad_value = props.pad_value.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();

                    let old_value = input.value();
                    let cursor_pos = input.selection_start().unwrap().unwrap_or(old_value.len() as u32);

                    let new_value = match radix {
                        Radix::Hex => padding_zero(&old_value.to_uppercase(), pad_value),
                        _ => padding_zero(&old_value.clone(), pad_value),
                    };

                    let new_cursor = (new_value.len() as i32 - (old_value.len() - cursor_pos as usize) as i32).max(0) as u32;
                    if new_value != old_value {
                        input.set_value(&new_value);

                        input.set_selection_start(Some(new_cursor)).ok();
                        input.set_selection_end(Some(new_cursor)).ok();
                    }

                    local_value.set(new_value.clone());
                    on_input.emit((new_value, radix.clone()));
                })
            }
        />
        </div>
        </td>
        <td>
        <input type="number" class="form-control" value={props.pad_value.to_string()} oninput={
            let on_pad_change = props.on_pad_change.clone();
            let radix = props.radix.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                if let Ok(v) = input.value().parse::<usize>() {
                    on_pad_change.emit((radix, v));
                }
            })
        } />
        </td>
    </tr>
    }
}

#[function_component(DigitalBase)]
pub fn digital_base(props: &HomeProps) -> Html {
    let lang = props.lang;
    let state = use_state(NumberState::new);

    let on_input = {
        let state = state.clone();
        Callback::from(move |(value, radix): (String, Radix)| {
            let mut new_state = (*state).clone();
            new_state.update_from_value(&value, radix);
            state.set(new_state);
        })
    };

    let on_pad_change = {
        let state = state.clone();
        Callback::from(move |(radix, value): (Radix, usize)| {
            let value = if value < 1 { 1 } else { value }; 
            let mut new_state = (*state).clone();
            match radix {
                Radix::Bin => new_state.bin_pad = value,
                Radix::Oct => new_state.oct_pad = value,
                Radix::Dec => new_state.dec_pad = value,
                Radix::Hex => new_state.hex_pad = value,
            }
            new_state.update_from_value(&state.decimal, Radix::Dec);
            state.set(new_state);
        })
    };

    html! {
        <>
        <Header route={Route::DigitalBase} {lang} />
        <BreadCrumb route={Route::DigitalBase} {lang} />
        <main class="container mt-2">
        <Title route={Route::DigitalBase} {lang} />
        <InvalidInput {lang} />
        <div class="row justify-content-md-center">
        <div class={class_core("")}>
        <div class="table-responsive">
        <table class="table align-middle">
        <thead>
            <tr>
            <th scope="col" style="width: 20%">{""}</th>
            <th scope="col" style="width: 50%">{_value(lang)}</th>
            <th scope="col" style="width: 30%">{_digits(lang)}</th>
            </tr>
        </thead>
        <tbody>
        <GroupedInput label={_bin(lang)} value={state.binary.clone()} radix={Radix::Bin} pad_value={state.bin_pad} group=4 on_input={on_input.clone()} on_pad_change={on_pad_change.clone()} />
        <GroupedInput label={_oct(lang)} value={state.octal.clone()} radix={Radix::Oct} pad_value={state.oct_pad} group=3 on_input={on_input.clone()} on_pad_change={on_pad_change.clone()} />
        <GroupedInput label={_dec(lang)} value={state.decimal.clone()} radix={Radix::Dec} pad_value={state.dec_pad} group=3 on_input={on_input.clone()} on_pad_change={on_pad_change.clone()} />
        <GroupedInput label={_hex(lang)} value={state.hexadecimal.clone()} radix={Radix::Hex} pad_value={state.hex_pad} group=2 on_input={on_input.clone()} on_pad_change={on_pad_change.clone()} />
        </tbody>
        </table>
        
        </div>
        </div>
        </div>
        </main>
        <Footer {lang} />
        </>
    }
}