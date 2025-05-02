use autocalc::digital::base::DigitalBase;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<DigitalBase>::with_props(HomeProps{ lang: Lang::Ja }).render();
}