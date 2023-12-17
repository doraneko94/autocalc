pub mod ja;
pub mod en;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{parse_query, encode_query};
use crate::url::{HOME, Lang, Url, make_path};

#[derive(Properties, PartialEq)]
pub struct HeaderBaseProps {
    pub title: String,
    pub lang_menu: String,
}

#[function_component(HeaderBase)]
pub fn header_base(props: &HeaderBaseProps) -> Html {
    let params = parse_query(use_location().unwrap().query_str());
    let mut params_ja = params.clone();
    let mut params_en = params.clone();
    params_ja.1 = Some(Lang::Ja);
    params_en.1 = Some(Lang::En);
    let query_ja = encode_query(params_ja);
    let query_en = encode_query(params_en);

    let lang = match params.1 {
        Some(lang) => lang,
        _ => Lang::En,
    };
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">
                <a href={HOME.to_url(lang)} class="navbar-brand">{&props.title}</a>
                <li class="nav-item dropdown">
                    <a class="nav-link dropdown-toggle navbar-brand" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                        {&props.lang_menu}
                    </a>
                    <ul class="dropdown-menu">
                        <li><a class="dropdown-item" href={make_path(query_en.as_str())}>{"🇬🇧 English"}</a></li>
                        <li><a class="dropdown-item" href={make_path(query_ja.as_str())}>{"🇯🇵 日本語"}</a></li>
                    </ul>
                </li>
            </div>
        </nav>
    }
}