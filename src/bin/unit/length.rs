use autocalc::unit::length::UnitLength;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<UnitLength>::with_props(HomeProps{ lang: Lang::Ja }).render();
}