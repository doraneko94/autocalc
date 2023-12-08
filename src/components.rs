pub mod header;
pub mod todo;

use yew::prelude::*;
use todo::todo_form::TodoForm;
use todo::todo_list::TodoList;
use todo::types::Todo;
use header::Header;

#[function_component(TodoHome)]
pub fn todo_home() -> Html {
    let todo_items = use_state(|| Vec::<Todo>::new());
    let next_id = use_state(|| 1);

    let on_add = {
        let todo_items = todo_items.clone();
        Callback::from(move |title: String| {
            let mut current_todo_items = (*todo_items).clone();
            current_todo_items.push(Todo {
                id: *next_id,
                title,
                completed: false,
            });
            next_id.set(*next_id + 1);
            todo_items.set(current_todo_items);
        })
    };
    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <TodoForm {on_add} />
                <TodoList todo_items={(*todo_items).clone()}/>
            </main>
        </>
    }
}