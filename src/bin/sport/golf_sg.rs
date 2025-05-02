use autocalc::sport::golf_sg::SportGolfSg;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<SportGolfSg>::with_props(HomeProps{ lang: Lang::Ja }).render();
}