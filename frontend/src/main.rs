use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class="text-sky-500 text-4xl font-bold">
        {"Hello world !"}
        </h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
