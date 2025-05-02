use autocalc::stat::roc_auc_ci::StatRocAucCi;
use autocalc::home::HomeProps;
use autocalc::router::Lang;

fn main() {
    yew::Renderer::<StatRocAucCi>::with_props(HomeProps{ lang: Lang::Ja }).render();
}