use yew::prelude::*;
use yew_router::prelude::*;

use crate::set_lang;
use crate::home::HomeProps;
use crate::meta::title_dscr;
use crate::router::{Lang, Route};

set_lang!(_ushitora, "艮電算術研究所", "Ushitora Lab.");
set_lang!(_ushitora_url, "https://ushitora.net", "https://ushitora.net");
set_lang!(_ushitora_dscr,
    "では、日常の問題を解決するための高度な計算プログラムを研究・設計・公開しています。自動計算サイトAutoCalcは、皆様に研究成果を手軽に試していただくために開設されました。",
    " researches, designs and publishes advanced calculation programmes to solve everyday problems. The 'AutoCalc' website was set up to make it easy for everyone to try out our research outputs."
);
set_lang!(_contact, "お問い合わせ・不具合報告", "Contact Us");
set_lang!(_contact_link, "https://ushitora.net/contact", "https://ushitora.net/en-GB/contact");

#[function_component(Footer)]
pub fn footer(props: &HomeProps) -> Html {
    let lang = props.lang;
    let privacy = match lang {
        Lang::Ja => Route::Privacy,
        Lang::En => Route::PrivacyEn
    };
    html! {
        <footer class="text-center text-lg-start bg-body-tertiary text-muted mt-auto py-3">
            <div class="text-center p-1" style="background-color: rgba(0, 0, 0, 0.05);">
                <div class="container">
                    <div class="row text-md-start mt-2">
                        <div class="col-md-6 col-lg-8 col-xl-6 mx-auto mb-2">
                            <h6 class="fw-bold mb-2">{_ushitora(lang)}</h6>
                            <p><a href={_ushitora_url(lang)}>{_ushitora(lang)}</a>{_ushitora_dscr(lang)}</p>
                        </div>
                        <div class="col-md-5 col-lg-4 col-xl-4 mx-auto mb-2">
                            <h6 class="fw-bold mb-2">{"Link"}</h6>
                            <p><Link<Route> to={privacy} classes="text-reset">{title_dscr(privacy).0}</Link<Route>></p>
                            <p><a href={_contact_link(lang)} class="text-reset">{_contact(lang)}</a></p>
                        </div>
                    </div>
                </div>
                <p>
                    {"© 2023 Copyright: "}
                    <a class="text-reset fw-bold" href={_ushitora_url(lang)}>{"ushitora.com"}</a>
                </p>
            </div>
        </footer>
    }
}