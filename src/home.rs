use yew::prelude::*;
use yew_router::prelude::*;

use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::layout::{class_home, class_text};
use crate::router::{Lang, Route};
use crate::title::Title;
use crate::meta::title_dscr;

#[derive(Properties, PartialEq)]
pub struct HomeTableProps {
    pub home: Route,
    pub pages: Vec<Route>,
    pub lang: Lang,
}

#[function_component(HomeTable)]
pub fn home_table(props: &HomeTableProps) -> Html {
    let (title, _) = title_dscr(props.home);
    html! {
        <table class="table table-bordered mb-2">
            <thead>
                <tr>
                    <th scope="col" style="width:8%">{"#"}</th>
                    <th scope="col" style="width:92%">{title}</th>
                </tr>
            </thead>
            <tbody>
                {
                    props.pages.iter().enumerate().map(|(i, &route)| html! {
                        <tr>
                        <th scope="row">{i+1}</th>
                        <td><Link<Route> to={route.clone()} classes="text-reset">{
                            title_dscr(route).0
                        }</Link<Route>></td>
                    </tr>
                    }).collect::<Html>()
                }
                <tr><td colspan="2"><Link<Route> to={props.home.clone()}>{
                    match props.lang {
                        Lang::Ja => "もっと見る",
                        Lang::En => "See more"
                    }
                }</Link<Route>></td></tr>
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct HomeCardProps {
    pub route: Route,
    pub lang: Lang,
}

#[function_component(HomeCard)]
pub fn home_card(props: &HomeCardProps) -> Html {
    let (title, dscr) = title_dscr(props.route);
    html! {
        <div class="card mb-2">
            <div class="card-body">
                <h5 class="card-title"><u>{title}</u></h5>
                <p class="card-text text-start">{dscr}</p>
                <Link<Route> to={props.route.clone()} classes="btn btn-primary">{
                    match props.lang {
                        Lang::Ja => "使ってみる",
                        Lang::En => "Try"
                    }
                }</Link<Route>>
            </div>
        </div>
    }
}

pub fn split_three_col<T: Sized + Clone>(pages: &[T]) -> (Vec<T>, Vec<T>, Vec<T>) {
    let size = pages.len();
    let n1 = size / 3 + if size % 3 > 0 { 1 } else { 0 };
    let n2 = n1 + size / 3 + if size % 3 == 2 { 1 } else { 0 };
    (pages[..n1].to_vec(), pages[n1..n2].to_vec(), pages[n2..].to_vec())
}

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub lang: Lang
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let mut lang = props.lang;
    let location = use_location().unwrap();
    let navigator = use_navigator().unwrap();
    match lang {
        Lang::Ja => {
            let (opt_page, opt_lang) = parse_query(location.query_str());
            lang = match opt_lang {
                Some(l) => l,
                None => Lang::Ja,
            };
            match opt_page {
                Some(p) => {
                    match p.as_str() {
                        "electronic" => { navigator.push(&Route::ElectronicHome.to_lang(lang)); },
                        "electronic/delta_y" => { navigator.push(&Route::ElectronicDeltaY.to_lang(lang)); },
                        "map" => { navigator.push(&Route::MapHome.to_lang(lang)); },
                        "map/circle_center" => { navigator.push(&Route::MapCircleCenter.to_lang(lang)); },
                        "math" => { navigator.push(&Route::MathHome.to_lang(lang)); },
                        "math/diffeq_linear2" => { navigator.push(&Route::MathDiffeqLinear2.to_lang(lang)); },
                        "math/diffeq_linear2_frac" => { navigator.push(&Route::MathDiffeqLinear2Frac.to_lang(lang)); },
                        "sport" => { navigator.push(&Route::SportHome.to_lang(lang)); },
                        "sport/golf_sg" => { navigator.push(&Route::SportGolfSg.to_lang(lang)); },
                        "stat" => { navigator.push(&Route::StatHome.to_lang(lang)); },
                        "stat/error_ellipse" => { navigator.push(&Route::StatErrorEllipse.to_lang(lang)); },
                        "stat/roc_auc_ci" => { navigator.push(&Route::StatRocAucCi.to_lang(lang)); },
                        "unit" => { navigator.push(&Route::UnitHome.to_lang(lang)); },
                        "unit/length" => { navigator.push(&Route::UnitLength.to_lang(lang)); },
                        "unit/mass" => { navigator.push(&Route::UnitMass.to_lang(lang)); },
                        "privacy" => { navigator.push(&Route::Privacy.to_lang(lang)); },
        
                        _ => {},
                    }
                }
                None => {}
            }
        }
        Lang::En => {}
    }
    
    let home_page = vec![
        (Route::UnitHome, vec![Route::UnitLength, Route::UnitMass]),
        (Route::MapHome, vec![Route::MapCircleCenter]),
        (Route::MathHome, vec![Route::MathDiffeqLinear2, Route::MathDiffeqLinear2Frac]),
        (Route::StatHome, vec![Route::StatRocAucCi, Route::StatErrorEllipse]),
        (Route::ElectronicHome, vec![Route::ElectronicDeltaY]),
        (Route::SportHome, vec![Route::SportGolfSg])
    ];
    let (c0, c1, c2) = split_three_col(&home_page);
    
    html! {
        <>
        <Header route={Route::Home} {lang} />
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
                            c0.iter().map(|(home, pages)| {
                                let (home, pages) = (home.to_lang(lang), pages.iter().map(|page| page.to_lang(lang)).collect::<Vec<Route>>());
                                html! { <><HomeTable {home} {pages} {lang} /></> }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class={class_home("")}>
                        {
                            c1.iter().map(|(home, pages)| {
                                let (home, pages) = (home.to_lang(lang), pages.iter().map(|page| page.to_lang(lang)).collect::<Vec<Route>>());
                                html! { <><HomeTable {home} {pages} {lang} /></> }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class={class_home("")}>
                        {
                            c2.iter().map(|(home, pages)| {
                                let (home, pages) = (home.to_lang(lang), pages.iter().map(|page| page.to_lang(lang)).collect::<Vec<Route>>());
                                html! { <><HomeTable {home} {pages} {lang} /></> }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </main>
        <Footer {lang} />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct HomeBaseProps {
    pub home: Route,
    pub pages: Vec<Route>,
    pub lang: Lang,
}

#[function_component(HomeBase)]
pub fn home_base(props: &HomeBaseProps) -> Html {
    let lang = props.lang;
    let home = props.home.to_lang(lang);
    let pages = props.pages.iter().map(|page| page.to_lang(lang)).collect::<Vec<Route>>();
    let size = pages.len();
    let n1 = size / 3 + if size % 3 > 0 { 1 } else { 0 };
    let n2 = n1 + size / 3 + if size % 3 == 2 { 1 } else { 0 };
    let p0 = pages[..n1].to_vec();
    let p1 = pages[n1..n2].to_vec();
    let p2 = pages[n2..].to_vec();
    html! {
        <>
        <Header route={home} {lang} />
        <Title route={home} {lang} />
        <BreadCrumb route={home} {lang} />
        <main class="container-fluid mt-2">
            <div class="container text-center">
                <div class="row align-items-start">
                    <div class={class_home("")}>
                        {
                            p0.iter().map(|route| html! {
                                <HomeCard route={route.clone()} {lang} />
                            }).collect::<Html>()
                        }
                    </div>
                    <div class={class_home("")}>
                        {
                            p1.iter().map(|route| html! {
                                <HomeCard route={route.clone()} {lang} />
                            }).collect::<Html>()
                        }
                    </div>
                    <div class={class_home("")}>
                        {
                            p2.iter().map(|route| html! {
                                <HomeCard route={route.clone()} {lang} />
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </main>
        <Footer {lang} />
        </>
    }
}

pub fn parse_query(query: &str) -> (Option<String>, Option<Lang>) {
    let query_replace = query.replace("?", "");
    let mut params: (Option<String>, Option<Lang>) = (None, None);
    let q_list: Vec<&str> = query_replace.split("&").collect();
    for q in q_list.iter() {
        let q_parts:Vec<&str> = q.split("=").collect();
        if q_parts.len() == 2 {
            match q_parts[0] {
                "p" => { params.0 = Some(q_parts[1].to_string()); }
                "lang" => {
                    params.1 = match q_parts[1] {
                        "ja" => Some(Lang::Ja),
                        _ => Some(Lang::En)
                    }
                }
                _ => {}
            }
        }
    }
    params
}