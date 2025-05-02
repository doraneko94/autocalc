use autocalc::electronic::delta_y::ElectronicDeltaY;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<ElectronicDeltaY>::with_props(HomeProps{ lang: Lang::En }).render();
}