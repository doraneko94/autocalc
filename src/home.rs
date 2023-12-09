pub mod ja;
pub mod en;

use yew::prelude::*;

use crate::router::make_path;

#[macro_export]
macro_rules! main_home {
    (
        $lang: expr, $title: expr,
        $length: expr, $mass: expr
    ) => {
        #[function_component(MainHome)]
        pub fn main_home() -> Html {
            html!{
                <>
                <Header />
                <AdsDisplayWide />
                <main class="container-fluid mt-2">
                    <MainHomeTable title={$title} items={
                        vec![
                            MainHomeTableItem::new($length, format!("{}{}", "?p=unit/length&lang=", $lang).as_str()),
                            MainHomeTableItem::new($mass, format!("{}{}", "?p=unit/mass&lang=", $lang).as_str())
                        ]
                    } />
                </main>
                <AdsDisplayWide />
                </>
            }
        }
    };
}

#[derive(Properties, PartialEq)]
pub struct MainHomeTableProps {
    pub title: String,
    pub items: Vec<MainHomeTableItem>,
}

#[function_component(MainHomeTable)]
pub fn main_home_table(props: &MainHomeTableProps) -> Html {
    html!{
        <table class="table">
            <thread>
                <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{props.title.clone()}</th>
                </tr>
            </thread>
            <tbody>
                {
                    props.items.iter().enumerate().map(|(i, item)| html! {
                        <tr>
                            <th scope="raw">{i+1}</th>
                            <td><a href={make_path(item.route.as_str())}>{item.value.clone()}</a></td>
                        </tr>
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}

#[derive(PartialEq, Clone)]
pub struct MainHomeTableItem {
    pub value: String,
    pub route: String,
}

impl MainHomeTableItem {
    pub fn new(value: &str, route: &str) -> Self {
        Self { value: value.to_string(), route: route.to_string() }
    }
}