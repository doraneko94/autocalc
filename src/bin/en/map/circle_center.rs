use autocalc::map::circle_center::MapCircleCenter;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<MapCircleCenter>::with_props(HomeProps{ lang: Lang::En }).render();
}