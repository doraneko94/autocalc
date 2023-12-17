pub mod ja;
pub mod en;

use yew::prelude::*;

use crate::url::{Lang, Url};

#[derive(Properties, PartialEq, Clone)]
pub struct MenuItemProps {
    pub url: String,
    pub name: String,
    pub lang: Lang,
}

#[function_component(MenuItem)]
pub fn menu_item(props: &MenuItemProps) -> Html {
    html! {
        <a href={props.url.to_url(props.lang)}>{&props.name}</a>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MenuProps {
    pub title: String,
    pub props_list: Vec<MenuItemProps>,
}

impl MenuProps {
    pub fn new(title: &str, lang: Lang, contents: &[(&str, &str)]) -> Self {
        let props_list = contents.iter().map(|(url, s)| {
            MenuItemProps { url: url.to_string(), name: s.to_string(), lang }
        }).collect();
        Self { title: title.to_string(), props_list }
    }
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    html! {
        <table class="table table-bordered">
            <thead>
                <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{props.title.clone()}</th>
                </tr>
            </thead>
            <tbody>
                {
                    props.props_list.iter().enumerate().map(|(i, props)| html! {
                        <tr>
                            <th scope="row">{i+1}</th>
                            <td><MenuItem 
                            url={props.url.clone()}
                            name={props.name.clone()}
                            lang={props.lang}/></td>
                        </tr>
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct MainHomeBaseProps {
    pub props_list_1: Vec<MenuProps>,
    pub props_list_2: Vec<MenuProps>,
    pub props_list_3: Vec<MenuProps>,
}

impl MainHomeBaseProps {
    pub fn new(props_list_1: &[MenuProps], props_list_2: &[MenuProps], props_list_3: &[MenuProps]) -> Self {
        Self {
            props_list_1: props_list_1.to_vec(),
            props_list_2: props_list_2.to_vec(),
            props_list_3: props_list_3.to_vec()
        }
    }
}

#[function_component(MainHomeBase)]
pub fn main_home_base(props: &MainHomeBaseProps) -> Html {
    html! {
        <main class="container-fluid mt-2">
            <div class="container text-center">
                <div class="row align-items-start">
                    <div class="col">
                        {
                            props.props_list_1.iter().map(|props| html! {
                                <Menu title={props.title.clone()} props_list={props.props_list.clone()} />
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="col">
                        {
                            props.props_list_2.iter().map(|props| html! {
                                <Menu title={props.title.clone()} props_list={props.props_list.clone()} />
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="col">
                        {
                            props.props_list_3.iter().map(|props| html! {
                                <Menu title={props.title.clone()} props_list={props.props_list.clone()} />
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </main>
    }
}

#[macro_export]
macro_rules! main_home {
    () => {
        #[function_component(MainHome)]
        pub fn main_home() -> Html {
            let props_list_1 = vec![
                MenuProps::new(UNIT_NAME, lang, &[
                    path_and_name(UNIT_LENGTH, lang),
                    path_and_name(UNIT_MASS, lang)
                ]),
            ];
            let props_list_2 = vec![
                MenuProps::new(MAP_NAME, lang, &[
                    path_and_name(MAP_CIRCLE_CENTER, lang),
                ]),
            ];
            let props_list_3 = vec![];
            html! {
                <>
                <Header />
                <MainHomeBase {props_list_1} {props_list_2} {props_list_3} />
                </>
            }
        }
    };
}