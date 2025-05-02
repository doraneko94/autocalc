use autocalc::stat::error_ellipse::StatErrorEllipse;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<StatErrorEllipse>::with_props(HomeProps{ lang: Lang::Ja }).render();
}