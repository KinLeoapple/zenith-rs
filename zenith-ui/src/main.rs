mod basic_components;

use crate::basic_components::input::ButtonInput;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ButtonInput input_type={"password"}/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
