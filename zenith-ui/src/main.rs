mod basic_components;
mod theme;

use gloo::console::log;
use crate::basic_components::input::Input;
use yew::prelude::*;
use crate::basic_components::button::Button;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Input input_type={"password"} end_decorator={
            html! {<Button is_decorator={true} text={"Hello"}/>}
        }/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
