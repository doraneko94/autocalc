use autocalc::digital::float::DigitalFloat;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<DigitalFloat>::with_props(HomeProps{ lang: Lang::En }).render();
}