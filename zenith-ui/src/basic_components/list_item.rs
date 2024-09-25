use yew::{classes, function_component, html, Callback, Html, Properties};
use crate::theme::size::Size;

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
    #[prop_or_default]
    pub on_click: Callback<()>, // on click event
}

#[function_component(ListItem)]
pub fn list_item(
    props: &ListItemProp
) -> Html {
    let width = format!("{}{}{}", "w-[", props.size.get().0 as f64 * 3.5, "px]");
    let height = format!("{}{}{}", "h-[", props.size.get().1, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-md") } else { None };
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };

    let onclick = {
        let on_click = props.on_click.clone();

        Callback::from(move |_| {
            on_click.emit(());
        })
    };

    html! {
        <li
            {onclick}
            key={props.key.clone()}
            class={classes!("flex", "items-center", "gap-2", width, height)}>
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