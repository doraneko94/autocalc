use autocalc::sport::SportHome;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<SportHome>::with_props(HomeProps{ lang: Lang::Ja }).render();
}