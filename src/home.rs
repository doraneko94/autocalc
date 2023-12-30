use yew::prelude::*;
use yew_router::prelude::*;

use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::layout::{class_home, class_text};
use crate::router::parse_query;
use crate::url::{self, DataMode, Lang};

#[derive(Properties, PartialEq)]
pub struct HomeTableProps {
    pub home: (String, String),
    pub pages: Vec<(String, String)>,
    pub lang: Lang,
}

#[function_component(HomeTable)]
pub fn home_table(props: &HomeTableProps) -> Html {
    html! {
        <table class="table table-bordered mb-2">
            <thead>
                <tr>
                    <th scope="col" style="width:8%">{"#"}</th>
                    <th scope="col" style="width:92%">{&props.home.0}</th>
                </tr>
            </thead>
            <tbody>
                {
                    props.pages.iter().enumerate().map(|(i, (name, url))| html! {
                        <tr>
                            <th scope="row">{i+1}</th>
                            <td><a href={url.clone()} class="text-reset">{name.clone()}</a></td>
                        </tr>
                    }).collect::<Html>()
                }
                <tr><td colspan="2"><a href={props.home.1.clone()}>{
                    match props.lang {
                        Lang::Ja => "もっと見る",
                        Lang::En => "See more"
                    }
                }</a></td></tr>
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct HomeCardProps {
    pub name: String,
    pub dscr: String,
    pub url: String,
    pub lang: Lang,
}

#[function_component(HomeCard)]
pub fn home_card(props: &HomeCardProps) -> Html {
    html! {
        <div class="card mb-2">
            <div class="card-body">
                <h5 class="card-title"><u>{&props.name}</u></h5>
                <p class="card-text text-start">{&props.dscr}</p>
                <a href={props.url.clone()} class="btn btn-primary">{
                    match props.lang {
                        Lang::Ja => "使ってみる",
                        Lang::En => "Try"
                    }
                }</a>
            </div>
        </div>
    }
}

pub fn make_table_page(f: fn(DataMode) -> String, lang: Lang) -> (String, String) {
    (f(DataMode::Name(lang)), f(DataMode::Url(lang)))
}

pub fn make_table_pages(values: &[fn(DataMode) -> String], lang: Lang) -> Vec<(String, String)> {
    values.iter().map(|f| {
        make_table_page(*f, lang)
    }).collect::<Vec<(String, String)>>()
}

pub fn make_card_pages(values: &[fn(DataMode) -> String], lang: Lang) -> Vec<(String, String, String)> {
    values.iter().map(|f| {
        (f(DataMode::Name(lang)), f(DataMode::Dscr(lang)), f(DataMode::Url(lang)))
    }).collect::<Vec<(String, String, String)>>()
}

pub fn split_three_col<T: Sized + Clone>(pages: &[T]) -> (Vec<T>, Vec<T>, Vec<T>) {
    let size = pages.len();
    let n1 = size / 3 + if size % 3 > 0 { 1 } else { 0 };
    let n2 = n1 + size / 3 + if size % 3 == 2 { 1 } else { 0 };
    (pages[..n1].to_vec(), pages[n1..n2].to_vec(), pages[n2..].to_vec())
}

#[function_component(MainHome)]
pub fn main_home() -> Html {
    let lang = match parse_query(use_location().unwrap().query_str()).1 {
        Some(Lang::Ja) => Lang::Ja, _ => Lang::En
    };
    let homes = vec![
        url::unit, url::map, url::math, url::stat, url::electronic, url::sport
    ];
    let pages = vec![
        vec![url::unit_length, url::unit_mass],
        vec![url::map_circle_center],
        vec![url::math_diffeq_linear2, url::math_diffeq_linear2_frac],
        vec![url::stat_roc_auc_ci, url::stat_error_ellipse],
        vec![url::electronic_delta_y],
        vec![url::sport_golf_sg]
    ];
    let (h0, h1, h2) = split_three_col(&homes);
    let (p0, p1, p2) = split_three_col(&pages);
    
    
    html! {
        <>
        <Header />
        <main class="container-fluid mt-2">
            <div class="container text-center">
                <div class="row justify-content-md-center mb-2">
                    <div class={class_text("")}>
                        {
                            match lang {
                                Lang::Ja => { html! { <><img src="/img/banner_ja.png" style="width:100%" /></> } }
                                Lang::En => { html! { <><img src="/img/banner_en.png" style="width:100%" /></> } }
                            }
                        }
                    </div>
                </div>
                <div class="row align-items-start">
                    <div class={class_home("")}>
                        {
                            h0.iter().zip(p0.iter()).map(|(f_home, f_pages)| {
                                let home = make_table_page(*f_home, lang);
                                let pages = make_table_pages(f_pages, lang);
                                html! { <><HomeTable {home} {pages} {lang} /></>}
                            }).collect::<Html>()
                        }
                    </div>
                    <div class={class_home("")}>
                        {
                            h1.iter().zip(p1.iter()).map(|(f_home, f_pages)| {
                                let home = make_table_page(*f_home, lang);
                                let pages = make_table_pages(f_pages, lang);
                                html! { <><HomeTable {home} {pages} {lang} /></>}
                            }).collect::<Html>()
                        }
                    </div>
                    <div class={class_home("")}>
                        {
                            h2.iter().zip(p2.iter()).map(|(f_home, f_pages)| {
                                let home = make_table_page(*f_home, lang);
                                let pages = make_table_pages(f_pages, lang);
                                html! { <><HomeTable {home} {pages} {lang} /></>}
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </main>
        <Footer />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct HomeBaseProps {
    pub pages: Vec<(String, String, String)>,
    pub lang: Lang,
}

#[function_component(HomeBase)]
pub fn home_base(props: &HomeBaseProps) -> Html {
    let size = props.pages.len();
    let n1 = size / 3 + if size % 3 > 0 { 1 } else { 0 };
    let n2 = n1 + size / 3 + if size % 3 == 2 { 1 } else { 0 };
    let p0 = props.pages[..n1].to_vec();
    let p1 = props.pages[n1..n2].to_vec();
    let p2 = props.pages[n2..].to_vec();
    html! {
        <>
        <Header />
        <BreadCrumb />
        <main class="container-fluid mt-2">
            <div class="container text-center">
                <div class="row align-items-start">
                    <div class={class_home("")}>
                        {
                            p0.iter().map(|(n, d, u)| html! {
                                <HomeCard name={n.clone()} dscr={d.clone()} url={u.clone()} lang={props.lang} />
                            }).collect::<Html>()
                        }
                    </div>
                    <div class={class_home("")}>
                        {
                            p1.iter().map(|(n, d, u)| html! {
                                <HomeCard name={n.clone()} dscr={d.clone()} url={u.clone()} lang={props.lang} />
                            }).collect::<Html>()
                        }
                    </div>
                    <div class={class_home("")}>
                        {
                            p2.iter().map(|(n, d, u)| html! {
                                <HomeCard name={n.clone()} dscr={d.clone()} url={u.clone()} lang={props.lang} />
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </main>
        <Footer />
        </>
    }
}