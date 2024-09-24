use yew::{classes, function_component, html, Callback, Html, Properties};
use crate::theme::color::{Color, Common, Text};
use crate::theme::size::Size;

#[derive(Properties, PartialEq)]
pub struct ButtonProp {
    #[prop_or(Size::Md)]
    pub size: Size, // size
    #[prop_or_default]
    pub text: String, // text
    #[prop_or(true)]
    pub rounded: bool, // border rounded
    #[prop_or(true)]
    pub shadow: bool, // show box shadow
    #[prop_or(false)]
    pub is_decorator: bool, // is decorator
    #[prop_or_default]
    pub start_decorator: Html, // start decorator
    #[prop_or_default]
    pub end_decorator: Html, // end decorator
    #[prop_or_default]
    pub on_click: Callback<()>, // on click event
}

#[function_component(Button)]
pub fn button(
    props: &ButtonProp
) -> Html {
    let width = format!("{}{}{}", "w-[", props.size.get().0, "px]");
    let height = format!("{}{}{}", "h-[", match props.is_decorator {
        true => props.size.get().1 - 10,
        false => props.size.get().1
    }, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-md") } else { None };
    let margin = match props.is_decorator {
        true => Some("mx-[3px]"),
        false => None
    };
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };

    let bg_color = format!("{}{}{}", "bg-[", Color::_500.primary(), "]");
    let bg_hover_color = format!("{}{}{}", "hover:bg-[", Color::_600.primary(), "]");
    let text_color = format!("{}{}{}", "text-[", Common::White.common(), "]");

    let onclick = {
        let on_click = props.on_click.clone();

        Callback::from(move |_| {
            on_click.emit(());
        })
    };

    html! {
        <div
            {onclick}
            class={classes!("select-none", "relative", "inline-flex", "items-center", "justify-center", "shrink-0", "transition-colors", "duration-100", "transition-colors", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "text-sm", "font-bold", bg_color, bg_hover_color, text_color, width, height, rounded, margin, shadow)}>
            { props.start_decorator.clone() }
            <button
                class={classes!("relative", "inline-flex", "items-center", "justify-center", "transition-colors", "duration-100", "transition-colors", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "text-sm", "font-bold")}>
                <span class="origin-left">{format!("{}", props.text)}</span>
            </button>
            { props.end_decorator.clone() }
        </div>

    }
}