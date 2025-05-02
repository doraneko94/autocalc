use autocalc::digital::bit_calc::DigitalBitCalc;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<DigitalBitCalc>::with_props(HomeProps{ lang: Lang::En }).render();
}