use yew::prelude::*;
use yew::Renderer;
//use yew::services::fetch::{FetchService, FetchTask, Request, Response};


#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <h1>{"Yew Todo App"}</h1>
            <p>{"This is a simple todo app built with Yew"}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}