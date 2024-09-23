use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProp {
    #[prop_or(100)]
    pub width: i32,
    #[prop_or(45)]
    pub height: i32,
    #[prop_or_default]
    pub text: String,
    #[prop_or(true)]
    pub rounded: bool,
    #[prop_or(true)]
    pub border: bool,
}

#[function_component(Button)]
pub fn button(
    props: &ButtonProp
) -> Html {
    let width = format!("{}{}{}", "w-[", props.width, "px]");
    let height = format!("{}{}{}", "h-[", props.height, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-lg") } else { None };
    let border = if props.border.clone() { Some("border-2") } else { None };

    html! {
        <button class={classes!("relative", "inline-flex", "items-center", "justify-center", "transition-colors", "duration-100", "border-solid", "transition-colors", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "bg-gray-800", "hover:bg-gray-700", "text-white", width, height, rounded, border)}>
            <span class="origin-left">{format!("{}", props.text)}</span>
        </button>
    }
}