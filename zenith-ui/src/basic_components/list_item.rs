use yew::{function_component, html, Callback, Html, Properties};
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
    html! {

    }
}