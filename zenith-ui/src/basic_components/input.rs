use yew::{classes, function_component, html, Html, Properties};
use crate::basic_components::button::Button;

#[derive(Properties, PartialEq)]
pub struct InputProp {
    #[prop_or(300)]
    pub width: i32,
    #[prop_or(45)]
    pub height: i32,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or(true)]
    pub rounded: bool,
    #[prop_or(true)]
    pub border: bool,
}

#[function_component(Input)]
pub fn input(
    props: &InputProp,
) -> Html {
    let width = format!("{}{}{}", "w-[", props.width, "px]");
    let height = format!("{}{}{}", "h-[", props.height, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-lg") } else { None };
    let border = if props.border.clone() { Some("border-2") } else { None };

    html! {
        <input
          class={classes!("bg-[#222630]", "relative", "inline-flex", "items-center", "justify-center", "px-4", "py-3", width, height, "outline-none", "text-white", rounded, border, "transition-colors", "duration-100", "border-solid", "focus:border-[#94a3b8]", "border-[#475569]")}
          placeholder={format!("{}", props.placeholder)}
          type={format!("{}", props.input_type)}
        />
    }
}

#[function_component(ButtonInput)]
pub fn button_input(
    props: &InputProp,
) -> Html {
    let width = format!("{}{}{}", "w-[", props.width, "px]");
    let height = format!("{}{}{}", "h-[", props.height, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-lg") } else { None };
    let border = if props.border.clone() { Some("border-2") } else { None };

    html! {
        <div class={classes!(width, height, rounded, border, "relative", "inline-flex", "items-center", "justify-center", "outline-none", "overflow-hidden", "transition-colors", "duration-100", "border-solid", "focus-within:border-[#94a3b8]", "border-[#475569]")}>
            <Input
                placeholder={format!("{}", props.placeholder)}
                rounded={false}
                border={false}/>
            <Button rounded={false} border={false}/>
        </div>
    }
}