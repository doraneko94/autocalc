use yew::prelude::*;
use crate::layout::class_text;
use crate::router::Lang;

#[derive(Properties, PartialEq)]
pub struct LinkHereProps {
    pub mes_ja: String,
    pub mes_en: String,
    pub url_ja: String,
    pub url_en: String,
    pub lang: Lang,
}

#[function_component(LinkHere)]
pub fn link_here(props: &LinkHereProps) -> Html {
    let lang = props.lang;
    html! {
        <div class="row justify-content-md-center">
            <div class={class_text("")}>
                {match lang {
                    Lang::Ja => html! {
                        <p>{&props.mes_ja}<a href={props.url_ja.clone()}>{"こちら"}</a>{"。"}</p>
                    },
                    Lang::En => html! {
                        <p>{&props.mes_en}<a href={props.url_en.clone()}>{"here"}</a>{"."}</p>
                    }
                }}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ReferenceProps {
    pub url_ja: String,
    pub url_en: String,
    pub lang: Lang,
}

#[function_component(Reference)]
pub fn reference(props: &ReferenceProps) -> Html {
    let lang = props.lang;
    html! {
        <div class="row justify-content-md-center">
            <div class={class_text("")}>
                {match lang {
                    Lang::Ja => html! {
                        <p>{"計算についての詳細は、"}<a href={props.url_ja.clone()}>{"こちら"}</a>{"を参照してください。"}</p>
                    },
                    Lang::En => html! {
                        <p>{"For more information on the calculations, see "}<a href={props.url_en.clone()}>{"here"}</a>{"."}</p>
                    }
                }}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub lang: Lang,
}

macro_rules! announce {
    ($name: ident, $ja: expr, $en: expr, $class: expr) => {
        #[function_component($name)]
        pub fn f(props: &ErrorProps) -> Html {
            let lang = props.lang;
            html! {
                <div class="row justify-content-md-center">
                    <div class={class_text("")}>
                        {match lang {
                            Lang::Ja => html! {
                                <p class={$class}>{$ja}</p>
                            },
                            Lang::En => html! {
                                <p class={$class}>{$en}</p>
                            }
                        }}
                    </div>
                </div>
            }
        }
    };
}

announce!(InvalidInput,
    "入力に異常な値が含まれる場合、計算は行われずに停止します。",
    "If the input contains invalid values, the calculation is not performed and stops.", ""
);

announce!(OnlyInteger,
    "入力値は整数にのみ対応しています。",
    "Input values can only be integers.", "text-danger"
);

announce!(InCaseDistortion,
    "平面座標系への変換に際しての誤差が大きくなる場合も、計算を停止します。（地点間の距離を100km以内とすることを推奨します）",
    "Calculations are also stopped if the error in converting to a planar coordinate system becomes significant. (It is recommended that the distance between locations should not exceed 100 km)", "text-danger"
);
