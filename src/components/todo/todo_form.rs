use yew::{function_component, html, Html, Callback, InputEvent, Properties, use_state, MouseEvent};

#[derive(Properties, PartialEq)]
pub struct TodoFromProps {
    pub on_add: Callback<String>,
}

#[function_component(TodoForm)]
pub fn todo_form(props: &TodoFromProps) -> Html {
    let title = use_state(|| "".to_string());

    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();

            match value {
                Some(value) => {
                    title.set((*title).clone() + &value);
                }
                None => {
                    title.set("".to_string());
                }
            }
        })
    };

    let onclick = {
        let on_add = props.on_add.clone();
        let title = title.clone();
        Callback::from(move |e: MouseEvent| {
            title.set("".to_string());
            e.prevent_default();
            on_add.emit((*title).clone());
        })
    };

    html! {
        <form class="mb-5">
            <table>
                <tr>
                    <td>{"緯度"}</td>
                    <td>
                        <input type="number" value={1} id="latitude" />
                    </td>
                    <td>{"経度"}</td>
                    <td>
                        <input type="number" value={1} id="longitude" />
                    </td>
                </tr>
            </table>
            <div class="mb-3">
                <label for="title" class="form-label">{"タイトル"}</label>
                <input type="text" value={(*title).clone()} {oninput} class="form-control" id="title" />
            </div>
            <div class="mb-3">
                {&*title}
            </div>
            <button type="submit" onclick={onclick} class="btn btn-primary">{"追加"}</button>
        </form>
    }
}