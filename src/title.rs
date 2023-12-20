use yew::prelude::*;

use crate::layout::class_text;

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