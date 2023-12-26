use yew::prelude::*;

use crate::layout::{class_core, class_text};

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub title: String,
    pub lead: String,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    html! {
        <div class="row justify-content-md-center">
            <div class={class_text("")}>
                <h1 class="mt-3">{&props.title}</h1>
                if props.lead != "".to_string() {
                    <p class="lead">{&props.lead}</p>
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