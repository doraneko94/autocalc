use autocalc::digital::DigitalHome;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<DigitalHome>::with_props(HomeProps{ lang: Lang::Ja }).render();
}