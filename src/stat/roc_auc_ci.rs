use yew::prelude::*;

use crate::utils::onchange_form;

#[derive(Properties, PartialEq)]
pub struct StatRocAucCiBaseProps {
    pub auc_name: String,
    pub pos_name: String,
    pub neg_name: String,
}

/*
#[function_component(StatRocAucCiBase)]
pub fn stat_roc_auc_base(props: &StatRocAucCiBaseProps) -> Html {
    let auc = use_state(|| 0.8.to_string());
    let pos = use_state(|| 20.to_string());
    let neg = use_state(|| 20.to_string());
    let percentile = use_state(|| 95.to_string());
    let lower = use_state(|| 0.to_string());
    let upper = use_state(|| 0.to_string());

    let onclick = {
        let auc = auc.clone();
        let pos = pos.clone();
        let neg = neg.clone();
        let lower = lower.clone();
        let upper = upper.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let auc = parse_state!(auc, f64);
            let pos = parse_state!(pos, f64);
            let neg = parse_state!(neg, f64);

            let center = match circle_center(p0, p1, p2) {
                Ok((center, _)) => center,
                Err(_) => return,
            };
            lat_c.set(center.lat().deg().to_string());
            lon_c.set(center.lon().deg().to_string());
        })
    };

    html! {
        <>
        <main class="container-fluid mt-2">
        <table class="table">
            <tbody>
                <tr>
                    <td>{&props.auc_name}</td>
                    <td colspan="3">
                        <input type="number" step=0.01 value={(*auc).clone()} onchange={onchange_form(auc.clone())} class="form-control" id="auc" />
                    </td>
                </tr>
                <tr>
                    <td>{&props.pos_name}</td>
                    <td><input type="number" step=1 value={(*pos).clone()} onchange={onchange_form(pos.clone())} class="form-control" id="pos" /></td>
                    <td>{&props.neg_name}</td>
                    <td><input type="number" step=1 value={(*neg).clone()} onchange={onchange_form(neg.clone())} class="form-control" id="neg" /></td>
                </tr>
                <tr>
                    <td>{&props.auc_name}</td>
                    <td colspan="3">
                        <input type="number" step=0.01 value={(*auc).clone()} onchange={onchange_form(auc.clone())} class="form-control" id="auc" />
                    </td>
                </tr>
                <tr>
                    <td>{"#"}</td>
                    <td>{"#"}</td>
                    <td><button type="submit" onclick={onclick} class="btn btn-primary">{props.calc.clone()}</button></td>
                </tr>
            </tbody>
        </table>
        </main>
        </>
    }
}
*/