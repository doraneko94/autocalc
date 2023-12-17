use yew::prelude::*;

use crate::header::HeaderBase;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <HeaderBase title={"自動計算サイト by 艮電算術研究所"} lang_menu={"言語設定"} />
        </>
    }
}