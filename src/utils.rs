use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

pub fn onchange_form(state: UseStateHandle<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let value = e.target_unchecked_into::<HtmlInputElement>().value();
        state.set(value);
    })
}

pub fn onchange_select(state: UseStateHandle<usize>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let value = e.target_unchecked_into::<HtmlSelectElement>().value();
        let n: usize = value.parse().unwrap();
        state.set(n);
    })
}

#[macro_export]
macro_rules! parse_state {
    ($name: ident, $type: ty) => {
        match (*$name).parse::<$type>() { Ok(v) => v, Err(_) => return, }
    };
}