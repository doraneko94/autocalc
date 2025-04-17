use yew::prelude::*;
use web_sys::window;

use crate::layout::{class_core, class_text};
use crate::meta::title_dscr;
use crate::router::{Lang, Route};

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub lang: Lang,
    pub route: Route,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let (title, dscr) = title_dscr(props.route.to_lang(props.lang));
    let document = window().unwrap().document().unwrap();
    document.set_title(&format!("{} | AutoCalc", &title));
    if let Some(meta) = document.query_selector("meta[name='description']").unwrap() {
        meta.set_attribute("content", &dscr).ok();
    }
    html! {
        <div class="row justify-content-md-center">
            <div class={class_text("")}>
                <h1 class="mt-3">{&title}</h1>
                if dscr != "".to_string() {
                    <p class="lead">{&dscr}</p>
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ThumbnailProps {
    pub img: String
}

#[function_component(Thumbnail)]
pub fn thumbnail(props: &ThumbnailProps) -> Html {
    html! {
        <div class="row justify-content-md-center">
            <div class={class_core("")}>
                <img src={props.img.clone()} class="img-fluid" />
            </div>
        </div>
    }
}