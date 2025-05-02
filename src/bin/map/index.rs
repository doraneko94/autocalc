use autocalc::map::MapHome;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<MapHome>::with_props(HomeProps{ lang: Lang::Ja }).render();
}