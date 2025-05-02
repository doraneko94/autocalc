use autocalc::unit::mass::UnitMass;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<UnitMass>::with_props(HomeProps{ lang: Lang::En }).render();
}