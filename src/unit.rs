pub mod length;
pub mod mass;

use yew::prelude::{Html, html, function_component};

#[function_component(UnitHome)]
pub fn unit_home() -> Html {
    html! {
        <table>
            <tr><a href="/length">{"長さ"}</a></tr>
            <tr><a href="/mass">{"質量"}</a></tr>
        </table>
    }
}