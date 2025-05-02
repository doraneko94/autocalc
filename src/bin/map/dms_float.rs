use autocalc::map::dms_float::MapDmsFloat;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<MapDmsFloat>::with_props(HomeProps{ lang: Lang::Ja }).render();
}