use crate::basic_components::context::theme_ctx::ThemeContext;
use crate::event::on_click::on_click;
use crate::theme::size::Size;
use crate::theme::theme_value::ThemeValue;
use yew::{classes, function_component, html, use_context, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ListItemProp {
    #[prop_or(Size::Md)]
    pub size: Size, // size
    #[prop_or(0)]
    pub key: i64, // the key of this list item
    #[prop_or_default]
    pub content: Html, // display content
    #[prop_or(true)]
    pub rounded: bool, // border rounded
    #[prop_or(true)]
    pub shadow: bool, // show box shadow
    #[prop_or_default]
    pub start_decorator: Html, // start decorator
    #[prop_or_default]
    pub end_decorator: Html, // end decorator
    #[prop_or(None)]
    pub on_click: Option<Callback<()>>, // on click event
}

#[function_component(ListItem)]
pub fn list_item(
    props: &ListItemProp
) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();

    let width = format!("{}{}{}", "w-[", props.size.get().0 as f64 * 3.5, "px]");
    let height = format!("{}{}{}", "h-[", props.size.get().1, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-md") } else { None };
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };
    let clickable = if props.on_click.is_some() { Some("cursor-pointer") } else { None };

    let bg_color = format!("{}{}{}", "bg-[", ThemeValue::Theme.list_items_bg_color(theme_ctx.inner.as_str()), "]");
    let bg_hover_color = format!("{}{}{}", "hover:bg-[", ThemeValue::Theme.list_items_hover_bg_color(theme_ctx.inner.as_str()), "]");
    let text_color = format!("{}{}{}", "text-[", ThemeValue::Theme.text_color(theme_ctx.inner.as_str()), "]");

    let onclick = on_click(props.on_click.clone(), None);

    html! {
        <li
            {onclick}
            key={props.key.clone()}
            class={classes!(clickable, "flex", "items-center", "gap-2", "transition-colors", "duration-100", bg_color, bg_hover_color, text_color, width, height, rounded, shadow, "overflow-hidden")}>
            <div class={classes!("flex-shrink-0")}>
                {props.start_decorator.clone()}
            </div>
            <div class={classes!("flex-1", "min-w-0")}>
                {props.content.clone()}
            </div>
            <div class={classes!("inline-flex", "items-center")}>
                {props.end_decorator.clone()}
            </div>
        </li>
    }
}