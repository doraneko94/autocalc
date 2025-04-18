use gloo::utils::document;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Lang, Route};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub lang: Lang,
    pub route: Route,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let lang = props.lang;

    document().document_element().unwrap().set_attribute(
        "lang", 
        match lang {
            Lang::Ja => "ja",
            Lang::En => "en"
        }
    ).unwrap();

    let route = props.route.to_lang(lang);
    let home = match lang {
        Lang::Ja => Route::Home,
        Lang::En => Route::HomeEn
    };
    let (title, lang_menu) = match lang {
        Lang::Ja => ("AutoCalc by 艮電算術研究所", "言語設定"),
        Lang::En => ("AutoCalc by Ushitora Lab.", "Languages"),
    };
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">
                <Link<Route> to={home} classes="navbar-brand">{title}</Link<Route>>
                <li class="nav-item dropdown">
                    <a class="nav-link dropdown-toggle navbar-brand" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                        {lang_menu}
                    </a>
                    <ul class="dropdown-menu">
                        <li><Link<Route> to={route.to_lang(Lang::Ja)} classes="dropdown-item">{"🇯🇵 日本語"}</Link<Route>></li>
                        <li><Link<Route> to={route.to_lang(Lang::En)} classes="dropdown-item">{"🇬🇧 English"}</Link<Route>></li>
                    </ul>
                </li>
            </div>
        </nav>
    }
}