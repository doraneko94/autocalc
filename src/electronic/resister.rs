use yew::prelude::*;

use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_core;
use crate::router::{Lang, Route};
use crate::set_lang;
use crate::title::Title;

set_lang!(_none, "無", "None");
set_lang!(_black, "黒", "Black");
set_lang!(_brown, "茶", "Brown");
set_lang!(_red, "赤", "Red");
set_lang!(_orange, "橙", "Orange");
set_lang!(_yellow, "黄", "Yellow");
set_lang!(_green, "緑", "Green");
set_lang!(_blue, "青", "Blue");
set_lang!(_purple, "紫", "Purple");
set_lang!(_gray, "灰", "Gray");
set_lang!(_white, "白", "White");
set_lang!(_gold, "金", "Gold");
set_lang!(_silver, "銀", "Silver");

#[derive(Clone, Copy)]
enum I8Mode {
    First,
    Second,
    Third,
    Exp,
}

#[derive(Clone)]
struct Resistance {
    pub first: Option<i8>,
    pub second: Option<i8>,
    pub third: Option<i8>,
    pub exp: Option<i8>,
    pub error: String,
}

impl Default for Resistance {
    fn default() -> Self {
        Self {
            first: Some(0),
            second: Some(0),
            third: None,
            exp: Some(0),
            error: "1%".to_string()
        }
    }
}

impl Resistance {
    fn show(&self) -> String {
        let r = match (self.first, self.second, self.third) {
            (Some(f), Some(s), Some(t)) => (f as u16 * 100 + s as u16 * 10 + t as u16).to_string(),
            (Some(f), Some(s), _) => (f as u16 * 10 + s as u16).to_string(),
            _ => { return "Error".to_string(); }
        };
        if let Some(ex) = self.exp {
            let c = (r.len() as i8 - 1 + ex) / 3;
            let u = match c {
                -1 => "m",
                0 => "",
                1 => "k",
                2 => "M",
                3 => "G",
                _ => { return "Error".to_string(); }
            };
            if let Ok(rf) = r.parse::<f32>() {
                let rs = format!("{:.2}", rf * 10f32.powi((ex - 3 * c) as i32)).trim_end_matches('0').trim_end_matches('.').to_string();
                return format!("{}{}Ω ± {}", rs, u, self.error);
            } else { return "Error".to_string(); }
        } else { return "Error".to_string(); }
    }
}

fn background_color(s: &str) -> String {
    let s = match s {
        "1%" => "background-color: #8B4513; color: white;",
        "2%" => "background-color: red; color: white;",
        "0.05%" => "background-color: orange; color: black;",
        "0.5%" => "background-color: green; color: white;",
        "0.25%" => "background-color: blue; color: white;",
        "0.1%" => "background-color: purple; color: white;",
        "5%" => "background-color: goldenrod; color: black;",
        "10%" => "background-color: silver; color: black;",
        _ => "background-color: white; color: black;",
    };
    s.to_string()
}

fn select_resistance<T: Fn(I8Mode)-> Callback<Event>>(f: T, mode: I8Mode, now: Option<i8>, lang: Lang) -> Html {
    let bg_color = match now {
        Some(0) => "background-color: black; color: white;",
        Some(1) => "background-color: #8B4513; color: white;",
        Some(2) => "background-color: red; color: white;",
        Some(3) => "background-color: orange; color: black;",
        Some(4) => "background-color: yellow; color: black;",
        Some(5) => "background-color: green; color: white;",
        Some(6) => "background-color: blue; color: white;",
        Some(7) => "background-color: purple; color: white;",
        Some(8) => "background-color: gray; color: black;",
        Some(-1) => "background-color: goldenrod; color: black;",
        Some(-2) => "background-color: silver; color: black;",
        _ => "background-color: white; color: black;",
    };
    html! {
        <select class="form-select" style={format!("font-size: 16px; padding: 6px; width: 100%; {}", bg_color)} onchange={f(mode)}>
            { match mode {
                I8Mode::Third => html! {
                    <>
                        <option value="none" style="background-color: white; color: black;" selected=true>{_none(lang)}</option>
                        <option value="0" style="background-color: black; color: white;">{_black(lang)}</option>
                    </>
                },
                _ => html! { <option value="0" style="background-color: black; color: white;" selected=true>{_black(lang)}</option> },
            } }
            <option value="1" style="background-color: #8B4513; color: white;">{_brown(lang)}</option>
            <option value="2" style="background-color: red; color: white;">{_red(lang)}</option>
            <option value="3" style="background-color: orange; color: black;">{_orange(lang)}</option>
            <option value="4" style="background-color: yellow; color: black;">{_yellow(lang)}</option>
            <option value="5" style="background-color: green; color: white;">{_green(lang)}</option>
            <option value="6" style="background-color: blue; color: white;">{_blue(lang)}</option>
            <option value="7" style="background-color: purple; color: white;">{_purple(lang)}</option>
            { match mode {
                I8Mode::Exp => html! {
                    <>
                        <option value="-3" style="background-color: white; color: black;">{_white(lang)}</option>
                        <option value="-1" style="background-color: goldenrod; color: black;">{_gold(lang)}</option>
                        <option value="-2" style="background-color: silver; color: black;">{_silver(lang)}</option>
                    </>
                },
                _ => html! {
                    <>
                        <option value="8" style="background-color: gray; color: black;">{_gray(lang)}</option>
                        <option value="9" style="background-color: white; color: black;">{_white(lang)}</option>
                    </>
                }
            } }
        </select>
    }
}

#[function_component(ElectronicResister)]
pub fn electronic_resister(props: &HomeProps) -> Html {
    let lang = props.lang;
    let resistance = use_state(Resistance::default);
    
    let onchange_i8 = |mode: I8Mode| {
        let resistance = resistance.clone();
        Callback::from(move |e: Event| {
            let mut r = (*resistance).clone();
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = match input.value().parse::<i8>() {
                Ok(i) => Some(i),
                Err(_) => None
            };
            match mode {
                I8Mode::First => { r.first = value; },
                I8Mode::Second => { r.second = value; },
                I8Mode::Third => { r.third = value; },
                I8Mode::Exp => { r.exp = value; },
            }
            resistance.set(r);
        }) 
    };

    let onchange_error = {
        let resistance = resistance.clone();
        Callback::from(move |e: Event| {
            let mut r = (*resistance).clone();
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            r.error = input.value();
            resistance.set(r);
        })
    };

    html! {
        <>
        <Header route={Route::ElectronicResister} {lang} />
        <BreadCrumb route={Route::ElectronicResister} {lang} />
        <main class="container mt-2">
        <Title route={Route::ElectronicResister} {lang} />
        <div class="row justify-content-md-center">
        <div class={class_core("")}>
        <div class="table-responsive">
        <table class="table align-middle" style="table-layout: fixed; width: 100%;">
        <thead>
            <th class="text-center" scope="col" style="width: 20%">{"1"}</th>
            <th class="text-center" scope="col" style="width: 20%">{"2"}</th>
            <th class="text-center" scope="col" style="width: 20%">{"(3)"}</th>
            <th class="text-center" scope="col" style="width: 20%">{"4"}</th>
            <th class="text-center" scope="col" style="width: 20%">{"5"}</th>
        </thead>
        <tbody>
            <tr>
                <td>{select_resistance(onchange_i8, I8Mode::First, (&*resistance).first, lang)}</td>
                <td>{select_resistance(onchange_i8, I8Mode::Second, (&*resistance).second, lang)}</td>
                <td>{select_resistance(onchange_i8, I8Mode::Third, (&*resistance).third, lang)}</td>
                <td>{select_resistance(onchange_i8, I8Mode::Exp, (&*resistance).exp, lang)}</td>
                <td>
                <select class="form-select" 
                    style={format!("font-size: 16px; padding: 6px; width: 100%; box-sizing: border-box; {}", background_color(&(&*resistance).error))} 
                    onchange={onchange_error}>
                    <option value="1%" style="background-color: #8B4513; color: white;" selected=true>{_brown(lang)}</option>
                    <option value="2%" style="background-color: red; color: white;">{_red(lang)}</option>
                    <option value="0.05%" style="background-color: orange; color: black;">{_orange(lang)}</option>
                    <option value="0.5%" style="background-color: green; color: white;">{_green(lang)}</option>
                    <option value="0.25%" style="background-color: blue; color: white;">{_blue(lang)}</option>
                    <option value="0.1%" style="background-color: purple; color: white;">{_purple(lang)}</option>
                    <option value="5%" style="background-color: goldenrod; color: black;">{_gold(lang)}</option>
                    <option value="10%" style="background-color: silver; color: black;">{_silver(lang)}</option>
                </select>
                </td>
            </tr>
            <tr>
                <td class="text-center" colspan="5">{&*resistance.show()}</td>
            </tr>
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