use crate::theme::color::Color;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DividerProp {
    #[prop_or(0)]
    pub inset: i32, // inset
    #[prop_or(Color::_600.neutral().to_string())]
    pub color: String, // the color of divider
    #[prop_or(false)]
    pub vertical: bool, // is vertical
}

#[function_component(Divider)]
pub fn divider(
    props: &DividerProp
) -> Html {
    let width = match props.vertical.clone() {
        true => "w-px",
        false => "w-full"
    };
    let height = match props.vertical.clone() {
        true => "h-full",
        false => "h-px"
    };
    let divider_width = match props.vertical.clone() {
        true => format!("{}{}{}", "h-[calc(100%-", props.inset * 2, "px)]"),
        false => format!("{}{}{}", "w-[calc(100%-", props.inset * 2, "px)]")
    };
    let divide = match props.vertical.clone() {
        true => "border-l",
        false => "border-b"
    };

    let border_color = format!("{}{}{}", "border-[", props.color.clone(), "]");

    html! {
        <div class={classes!(width, height, "relative", "inline-flex", "items-center", "justify-center")}>
            <div class={classes!(divider_width, divide, border_color)}/>
        </div>
    }
}