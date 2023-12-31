use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::parse_query;
use crate::url::*;

#[derive(Properties, PartialEq)]
pub struct BreadCrumbBaseProps {
    pub lang: Lang,
    pub current: String,
    pub parent: Vec<(String, String)>
}

#[function_component(BreadCrumbBase)]
pub fn breadcrumb_base(props: &BreadCrumbBaseProps) -> Html {
    html!{
        <div class="container">
        <nav style="--bs-breadcrumb-divider: '>';" aria-label="breadcrumb">
            <ol class="breadcrumb">
                {
                    props.parent.iter().map(|(url, name)| html! {
                        <li class="breadcrumb-item"><a href={url.clone()}>{name}</a></li>
                    }).collect::<Html>()
                }
                <li class="breadcrumb-item active" aria-current="page">{props.current.clone()}</li>
            </ol>
        </nav>
        </div>
    }
}

#[function_component(BreadCrumb)]
pub fn breadcrumb() -> Html {
    let params = parse_query(use_location().unwrap().query_str());
    let lang = match params {
        (_, Some(lang)) => lang,
        (_, None) => Lang::En
    };
    
    let (current, parent) = match params {
        (None, _) => { (home(DataMode::Name(lang)), vec![]) }
        (Some(path), _) => {
            let current = path_to_name(&path, lang);
            let path_list = path.split("/").collect::<Vec<&str>>();
            let length = path_list.len();
            let parent = (0..length).map(|i| {
                let mut path = "".to_string();
                for j in 0..i {
                    if j > 1 { path = path + "/"; }
                    path = path + path_list[j];
                }
                (path_to_url(&path, lang), path_to_name(&path, lang))
            }).collect();
            (current, parent)
        }
    };

    html! {
        <><BreadCrumbBase {lang} {current} {parent} /></>
    }
}