use autocalc::math::diffeq_linear2_frac::MathDiffeqLinear2Frac;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<MathDiffeqLinear2Frac>::with_props(HomeProps{ lang: Lang::Ja }).render();
}