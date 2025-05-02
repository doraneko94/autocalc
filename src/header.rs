use yew::prelude::*;

use crate::router::{Lang, Route};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub lang: Lang,
    pub route: Route,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let lang = props.lang;

    let (title, lang_menu) = match lang {
        Lang::Ja => ("AutoCalc by 艮電算術研究所", "言語設定"),
        Lang::En => ("AutoCalc by Ushitora Lab.", "Languages"),
    };
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">
                <a href={Route::Home.to_url(lang)} class="navbar-brand">{title}</a>
                <li class="nav-item dropdown">
                    <a class="nav-link dropdown-toggle navbar-brand" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                        {lang_menu}
                    </a>
                    <ul class="dropdown-menu">
                        <li><a href={props.route.to_url(Lang::Ja)} class="dropdown-item">{"🇯🇵 日本語"}</a></li>
                        <li><a href={props.route.to_url(Lang::En)} class="dropdown-item">{"🇬🇧 English"}</a></li>
                    </ul>
                </li>
            </div>
        </nav>
    }
}