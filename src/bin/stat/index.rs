use autocalc::stat::StatHome;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<StatHome>::with_props(HomeProps{ lang: Lang::Ja }).render();
}