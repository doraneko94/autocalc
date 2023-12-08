use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">
                <a class="navbar-brand" href="#">{"自動計算サイト by 艮電算術研究所"}</a>
            </div>
        </nav>
    }
}