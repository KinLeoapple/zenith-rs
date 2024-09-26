mod basic_components;
mod theme;
mod event;

use yew::prelude::*;
use crate::basic_components::list_item::ListItem;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ListItem
            content={
            html! {
                <>
                    <p class="text-sm font-medium text-gray-900 truncate dark:text-white">
               {"Neil Sims"}
            </p>
            <p class="text-sm text-gray-500 truncate dark:text-gray-400">
               {"email@flowbite.com"}
            </p>
                </>
            }
        }/>
        // <List list_items={vec![html!{<p>{"hello"}</p>}, html!{<p>{"hello"}</p>}]}
        //     list_gap={2}
        //     divider={
        //     html!{<Divider inset={2}/>}
        // }/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
