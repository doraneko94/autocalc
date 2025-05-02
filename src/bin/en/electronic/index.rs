use autocalc::electronic::ElectronicHome;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<ElectronicHome>::with_props(HomeProps{ lang: Lang::En }).render();
}