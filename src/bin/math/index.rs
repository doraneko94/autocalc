use autocalc::math::MathHome;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<MathHome>::with_props(HomeProps{ lang: Lang::Ja }).render();
}