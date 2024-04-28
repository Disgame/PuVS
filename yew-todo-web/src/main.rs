use yew::prelude::*;
use yew::Renderer;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{"Yew Todo App"}</h1>
            <p>{"This is a simple todo app built with Yew"}</p>
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}