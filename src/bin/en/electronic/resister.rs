use autocalc::electronic::resister::ElectronicResister;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<ElectronicResister>::with_props(HomeProps{ lang: Lang::En }).render();
}