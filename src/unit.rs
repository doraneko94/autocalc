pub mod length;
pub mod weight;

use yew::prelude::{Html, html, function_component};

#[function_component(UnitHome)]
pub fn unit_home() -> Html {
    html! {
        <table>
            <tr><a href="/length">{"長さ"}</a></tr>
            <tr><a href="/weight">{"重さ"}</a></tr>
        </table>
    }
}