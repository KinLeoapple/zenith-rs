mod basic_components;
mod theme;
mod event;

use crate::basic_components::card::Card;
use yew::prelude::*;
use crate::basic_components::avatar::Avatar;
use crate::basic_components::button::Button;
use crate::basic_components::input::Input;
use crate::basic_components::image::Image;

#[function_component(App)]
fn app() -> Html {
    let mut vec: Vec<Html> = Vec::new();
    vec.push(html!(<Button/>));
    vec.push(html!(<Button/>));
    vec.push(html!(<Button/>));

    html! {
        <>
            <Avatar src={"https://images.unsplash.com/photo-1527549993586-dff825b37782?auto=format&fit=crop&w=286"}/>
            <Card title="This is the main card" content={"Hello"} actions={vec} image_src={
            "https://images.unsplash.com/photo-1527549993586-dff825b37782?auto=format&fit=crop&w=286"}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
