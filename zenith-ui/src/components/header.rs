use crate::basic_components::data_display::list::List;
use crate::basic_components::data_display::typography::Typography;
use crate::basic_components::image::Image;
use crate::theme::size::Size;
use yew::{classes, function_component, html, Callback, Html, Properties};
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderButtonProp {
    #[prop_or_default]
    pub text: String, // text
    #[prop_or(Route::Home)]
    pub route: Route // route
}

#[function_component(HeaderButton)]
fn header_button(
    props: &HeaderButtonProp
) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = {
        let route = props.route.clone();

        Callback::from(move |_| navigator.replace(&route))
    };

    html! {
        <Typography text={format!("{}", props.text)} on_click={onclick} />
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    let mut list_items: Vec<Html> = vec![
        html!(<HeaderButton text={"Home"} route={Route::Home}/>),
        html!(<HeaderButton text={"Blog"} route={Route::Blog}/>),
        html!(<HeaderButton text={"Image"} route={Route::Image}/>)
    ];

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