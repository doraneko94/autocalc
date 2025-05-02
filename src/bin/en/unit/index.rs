use autocalc::unit::UnitHome;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<UnitHome>::with_props(HomeProps{ lang: Lang::En }).render();
}