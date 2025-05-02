use autocalc::math::diffeq_linear2::MathDiffeqLinear2;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<MathDiffeqLinear2>::with_props(HomeProps{ lang: Lang::En }).render();
}