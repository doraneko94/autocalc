use autocalc::home::{Home, HomeProps};
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<Home>::with_props(HomeProps{ lang: Lang::En }).render();
}