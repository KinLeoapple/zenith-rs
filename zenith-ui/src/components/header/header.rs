use crate::basic_components::data_display::list::List;
use crate::basic_components::data_display::typography::Typography;
use crate::basic_components::image::Image;
use crate::theme::size::Size;
use yew::{classes, function_component, html, use_context, use_state, Callback, Html, Properties};
use yew_router::prelude::*;
use crate::basic_components::icon::Icon;
use crate::basic_components::context::theme_ctx::ThemeContext;
use crate::Route;
use crate::theme::color::Theme;

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

#[function_component(ThemeIcon)]
fn theme_icon() -> Html {
    let svg_list = vec![
        html!(<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-brightness-high-fill" viewBox="0 0 16 16">
                  <path d="M12 8a4 4 0 1 1-8 0 4 4 0 0 1 8 0M8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0m0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13m8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5M3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8m10.657-5.657a.5.5 0 0 1 0 .707l-1.414 1.415a.5.5 0 1 1-.707-.708l1.414-1.414a.5.5 0 0 1 .707 0m-9.193 9.193a.5.5 0 0 1 0 .707L3.05 13.657a.5.5 0 0 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0m9.193 2.121a.5.5 0 0 1-.707 0l-1.414-1.414a.5.5 0 0 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707M4.464 4.465a.5.5 0 0 1-.707 0L2.343 3.05a.5.5 0 1 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .708"/>
                </svg>), // Light Icon
        html!(<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-moon-stars-fill" viewBox="0 0 16 16">
                  <path d="M6 .278a.77.77 0 0 1 .08.858 7.2 7.2 0 0 0-.878 3.46c0 4.021 3.278 7.277 7.318 7.277q.792-.001 1.533-.16a.79.79 0 0 1 .81.316.73.73 0 0 1-.031.893A8.35 8.35 0 0 1 8.344 16C3.734 16 0 12.286 0 7.71 0 4.266 2.114 1.312 5.124.06A.75.75 0 0 1 6 .278"/>
                  <path d="M10.794 3.148a.217.217 0 0 1 .412 0l.387 1.162c.173.518.579.924 1.097 1.097l1.162.387a.217.217 0 0 1 0 .412l-1.162.387a1.73 1.73 0 0 0-1.097 1.097l-.387 1.162a.217.217 0 0 1-.412 0l-.387-1.162A1.73 1.73 0 0 0 9.31 6.593l-1.162-.387a.217.217 0 0 1 0-.412l1.162-.387a1.73 1.73 0 0 0 1.097-1.097zM13.863.099a.145.145 0 0 1 .274 0l.258.774c.115.346.386.617.732.732l.774.258a.145.145 0 0 1 0 .274l-.774.258a1.16 1.16 0 0 0-.732.732l-.258.774a.145.145 0 0 1-.274 0l-.258-.774a1.16 1.16 0 0 0-.732-.732l-.774-.258a.145.145 0 0 1 0-.274l.774-.258c.346-.115.617-.386.732-.732z"/>
                </svg>) // Dark Icon
    ];

    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let svg_handle = use_state(||
        if theme_ctx.inner.clone() == Theme::Light.theme() {
            svg_list.get(0).unwrap().clone()
        } else {
            svg_list.get(1).unwrap().clone()
        }
    );
    let svg = (*svg_handle).clone();

    let onclick = {
        let theme_ctx = theme_ctx.clone();
        let svg_handle = svg_handle.clone();

        Callback::from(move |_| {
            if theme_ctx.inner == Theme::Light.theme() {
                theme_ctx.dispatch(Theme::Dark.theme().to_string());
                svg_handle.set(svg_list.get(1).unwrap().clone());
            } else {
                theme_ctx.dispatch(Theme::Light.theme().to_string());
                svg_handle.set(svg_list.get(0).unwrap().clone());
            }
        })
    };

    html! {
        <Icon svg={svg.clone()} on_click={onclick} />
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    let list_items: Vec<Html> = vec![
        html!(<HeaderButton text={"Home"} route={Route::Home}/>),
        html!(<HeaderButton text={"Blog"} route={Route::Blog}/>),
        html!(<HeaderButton text={"Image"} route={Route::Image}/>),
        html!(<ThemeIcon/>)
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