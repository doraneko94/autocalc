use yew::prelude::*;

use crate::header::HeaderBase;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <HeaderBase title={"AutoCalc by Ushitora Lab."} lang_menu={"Languages"} />
        </>
    }
}