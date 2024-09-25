mod basic_components;
mod theme;

use yew::prelude::*;
use crate::basic_components::divider::Divider;
use crate::basic_components::list::List;

#[function_component(App)]
fn app() -> Html {
    html! {
        <List list_items={vec![html!{<p>{"hello"}</p>}, html!{<p>{"hello"}</p>}]}
            list_gap={2}
            divider={
            html!{<Divider inset={50}/>}
        }/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
