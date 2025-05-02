use autocalc::privacy::Privacy;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<Privacy>::with_props(HomeProps{ lang: Lang::Ja }).render();
}