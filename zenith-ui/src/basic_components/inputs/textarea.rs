use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};
use crate::theme::size::Size;
use yew::{classes, function_component, html, use_node_ref, use_state, Callback, Html, Properties};
use crate::theme::color::{Color, Text};

#[derive(Properties, PartialEq)]
pub struct TextareaProp {
    #[prop_or(Size::Md)]
    pub size: Size,
    #[prop_or_default]
    pub default_value: String, // default value
    #[prop_or_default]
    pub placeholder: String, // placeholder
    #[prop_or(100)]
    pub max_length: i32, // max length
    #[prop_or(true)]
    pub rounded: bool, // border rounded
    #[prop_or(true)]
    pub border: bool, // show border
    #[prop_or(true)]
    pub shadow: bool, // show box shadow
    #[prop_or_default]
    pub label: String, // textarea label
    #[prop_or(5)]
    pub rows: i32
}

#[function_component(Textarea)]
pub fn textarea(
    props: &TextareaProp,
) -> Html {
    let width = format!("{}{}{}", "w-[", props.size.get().0 as f64 * 3.5, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-md") } else { None };
    let border = if props.border.clone() { Some("border") } else { None };
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };

    let bg_color = format!("{}{}{}", "bg-[", Color::_900.primary(), "]");
    let text_color = format!("{}{}{}", "text-[", Text::Primary.dark(), "]");
    let border_color = format!("{}{}{}", "border-[", Color::_500.primary(), "]");
    let border_focus_color = format!("{}{}{}", "focus-within:border-[", Color::_200.primary(), "]");

    let textarea_node_ref = use_node_ref();
    let textarea_value_handle = use_state(|| format!("{}", props.default_value));
    let textarea_value = (*textarea_value_handle).clone();

    let oninput = {
        let textarea_node_ref = textarea_node_ref.clone();
        let textarea_value_handle = textarea_value_handle.clone();

        Callback::from(move |_| {
            let input = textarea_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                textarea_value_handle.set(input.value());
            }
        })
    };

    let onchange = {
        let textarea_value_handle = textarea_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                textarea_value_handle.set(input.value());
            }
        })
    };

    html! {
        <>
            if props.label.chars().count() > 0 {
                <label for={"message"} class={classes!("block", "mb-2", text_color.clone())}>{props.label.clone()}</label>
            }
            <textarea
                ref={textarea_node_ref}
                value={textarea_value.clone()}
                placeholder={format!("{}", props.placeholder)}
                maxLength={format!("{}", props.max_length)}
                {oninput}
                {onchange}
                id={"message"}
                rows={props.rows.clone().to_string()}
                class={classes!("resize-none", "block", "p-2.5", width, "text-sm", text_color.clone(), bg_color, rounded, border, border_color, border_focus_color, shadow)}/>
        </>
    }
}