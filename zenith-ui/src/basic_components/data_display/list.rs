use crate::basic_components::context::theme_ctx::ThemeContext;
use crate::theme::theme_value::ThemeValue;
use yew::{classes, function_component, html, use_context, use_state, Html, Properties};
use crate::basic_components::scrollbar::Scrollbar;

#[derive(Properties, PartialEq)]
pub struct ListProp {
    #[prop_or(2)]
    pub list_gap: i32, // the gap of each item
    #[prop_or(true)]
    pub vertical: bool, // is vertical
    #[prop_or_default]
    pub divider: Html, // divider of the list
    #[prop_or_default]
    pub list_items: Vec<Html>, // the list of list items
}

#[function_component(List)]
pub fn list(
    props: &ListProp
) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();

    let vertical = match props.vertical.clone() {
        true => Some("flex-col"),
        false => None
    };
    let list_gap = format!("{}{}", "gap-", props.list_gap);

    let text_color = format!("{}{}{}", "text-[", ThemeValue::Theme.text_color(theme_ctx.inner.as_str()), "]");

    let list_items_handle = use_state(|| props.list_items.clone());
    let list_item = (*list_items_handle).clone();

    html! {
        <Scrollbar layer_top={true}>
            <ul class={classes!("text-sm", "font-bold", "w-fit", "h-fit", "flex", vertical, list_gap, "items-center", "justify-center", "leading-normal", text_color)}>
                {list_item.iter().enumerate().map(|(index, html)| {
                    html! {
                        <>
                            {html.clone()}
                            if index < list_item.len() - 1 {
                                {props.divider.clone()}
                            }
                        </>
                    }
                }).collect::<Html>()}
            </ul>
        </Scrollbar>
    }
}