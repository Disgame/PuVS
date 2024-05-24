use std::array;

use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Renderer;
use yew::Properties;
use wasm_bindgen_futures::spawn_local; // Import spawn_local for executing async functions in the browser
use yew::use_effect_with;
mod routes;
use web_sys::wasm_bindgen::JsCast;
#[derive(Properties, Clone, PartialEq)]
pub struct ListProperties {
    pub todos: Vec<String>,
    pub on_done: Callback<usize>,
}

#[function_component(List)]
fn list(props: &ListProperties) -> Html {
    if let todos = &props.todos {
        html! {
            <ul>
                { for todos.iter().map(|todo| html! { <li>{todo}</li> }) }
            </ul>
        }
    } else {    
        html! {
            <p>{"Loading..."}</p>
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let add_todo = {
        let todos = use_state(|| vec![]);

        let todos_clone = todos.clone();
        Callback::from(move |todo_name: String| {
            let mut todos_c_clone = todos.clone();
            todos_c_clone.set(vec![format!("[{}]", todo_name)]);
        })
    };
    
    



    html! {
        <div>
            <h1>{"Yew Todo App"}</h1>

        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}