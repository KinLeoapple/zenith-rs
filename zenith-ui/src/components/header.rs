use crate::basic_components::data_display::list::List;
use crate::basic_components::data_display::typography::Typography;
use crate::basic_components::image::Image;
use crate::theme::size::Size;
use yew::{classes, function_component, html, Callback, Html};
use yew_router::prelude::*;
use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let mut list_items: Vec<Html> = Vec::new();
    list_items.push(html!(<Typography text={"Home"}/>));
    list_items.push(html!(<Typography text={"Blog"}/>));
    list_items.push(html!(<Typography text={"Image"}/>));

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.replace(&Route::Home));

    let height_num = Size::Md.get().1 as f64 * 1.5;
    let height = format!("{}{}{}", "h-[", height_num, "px]");

    html! {
        <div class={classes!("fixed", "top-0", "left-0", "flex", "justify-between", "w-full", height.clone())}>
            <div class={classes!("flex", "justify-between", "items-center", "ml-10", height.clone())}>
                <Image src="./assets/logo.png" height={(height_num * 0.8) as i32} on_click={onclick}/>
            </div>
            <div class="flex justify-between items-center mr-10">
                <List vertical={false} list_gap={6} list_items={list_items}/>
            </div>
        </div>
    }
}