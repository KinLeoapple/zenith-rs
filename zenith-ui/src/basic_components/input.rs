use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};
use yew::{classes, function_component, html, use_node_ref, use_state, Callback, Html, Properties};
use crate::theme::color::{Color, Text};
use crate::theme::size::Size;

#[derive(Properties, PartialEq)]
pub struct InputProp {
    #[prop_or(Size::Md)]
    pub size: Size,
    #[prop_or_default]
    pub default_value: String, // default value
    #[prop_or_default]
    pub placeholder: String, // placeholder
    #[prop_or(100)]
    max_length: i32, // max length
    #[prop_or_default]
    pub input_type: String, // input type
    #[prop_or(true)]
    pub rounded: bool, // border rounded
    #[prop_or(true)]
    pub border: bool, // show border
    #[prop_or(true)]
    pub shadow: bool, // show box shadow
    #[prop_or_default]
    pub start_decorator: Html, // start decorator
    #[prop_or_default]
    pub end_decorator: Html // end decorator
}

#[function_component(Input)]
pub fn input(
    props: &InputProp,
) -> Html {
    let width = format!("{}{}{}", "w-[", props.size.get().0 as f64 * 3.5, "px]");
    let height = format!("{}{}{}", "h-[", props.size.get().1, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-md") } else { None };
    let border = if props.border.clone() { Some("border") } else { None };
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };

    let bg_color = format!("{}{}{}", "bg-[", Color::_900.primary(), "]");
    let text_color = format!("{}{}{}", "text-[", Text::Primary.dark(), "]");
    let border_color = format!("{}{}{}", "border-[", Color::_500.primary(), "]");
    let border_focus_color = format!("{}{}{}", "focus-within:border-[", Color::_200.primary(), "]");

    let input_node_ref = use_node_ref();
    let input_value_handle = use_state(|| format!("{}", props.default_value));
    let input_value = (*input_value_handle).clone();

    let input_type_handle = use_state(|| format!("{}", props.input_type));
    let input_type = (*input_type_handle).clone();

    let pass_visible_handle = use_state(|| false);
    let pass_visible = (*pass_visible_handle).clone();

    let oninput = {
        let input_node_ref = input_node_ref.clone();
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let onchange = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let onclick = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |_| {
            input_value_handle.set(String::default());
        })
    };

    let change_pass_visible = {
        let input_type_handle = input_type_handle.clone();
        let pass_visible_handle = pass_visible_handle.clone();

        Callback::from(move |_| {
            if pass_visible {
                input_type_handle.set("password".to_string());
                pass_visible_handle.set(false);
            } else {
                input_type_handle.set("text".to_string());
                pass_visible_handle.set(true);
            }
        })
    };

    html! {
        <div class={classes!(bg_color.clone(), "select-none", "relative", "inline-flex", "items-center", "justify-center", width.clone(), height.clone(), "outline-offset-0", text_color.clone(), rounded, border, "transition-colors", "duration-100", "border-solid", border_focus_color, border_color, shadow, "overflow-hidden")}>
            { props.start_decorator.clone() }
            <input
                ref={input_node_ref}
                {oninput}
                {onchange}
                class={classes!(bg_color.clone(), text_color.clone(), "w-full", "h-full", "outline-none", "px-4", "pl-3", "pr-1")}
                placeholder={format!("{}", props.placeholder)}
                maxLength={format!("{}", props.max_length)}
                type={input_type.clone()}
                value={input_value.clone()}
            />
            if props.input_type == "password" {
                <button
                    onclick={change_pass_visible}
                    class={classes!("mr-3")}>
                    if pass_visible.clone() {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-eye" viewBox="0 0 16 16">
                          <path d="M16 8s-3-5.5-8-5.5S0 8 0 8s3 5.5 8 5.5S16 8 16 8M1.173 8a13 13 0 0 1 1.66-2.043C4.12 4.668 5.88 3.5 8 3.5s3.879 1.168 5.168 2.457A13 13 0 0 1 14.828 8q-.086.13-.195.288c-.335.48-.83 1.12-1.465 1.755C11.879 11.332 10.119 12.5 8 12.5s-3.879-1.168-5.168-2.457A13 13 0 0 1 1.172 8z"/>
                          <path d="M8 5.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5M4.5 8a3.5 3.5 0 1 1 7 0 3.5 3.5 0 0 1-7 0"/>
                        </svg>
                    } else {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-eye-slash" viewBox="0 0 16 16">
                          <path d="M13.359 11.238C15.06 9.72 16 8 16 8s-3-5.5-8-5.5a7 7 0 0 0-2.79.588l.77.771A6 6 0 0 1 8 3.5c2.12 0 3.879 1.168 5.168 2.457A13 13 0 0 1 14.828 8q-.086.13-.195.288c-.335.48-.83 1.12-1.465 1.755q-.247.248-.517.486z"/>
                          <path d="M11.297 9.176a3.5 3.5 0 0 0-4.474-4.474l.823.823a2.5 2.5 0 0 1 2.829 2.829zm-2.943 1.299.822.822a3.5 3.5 0 0 1-4.474-4.474l.823.823a2.5 2.5 0 0 0 2.829 2.829"/>
                          <path d="M3.35 5.47q-.27.24-.518.487A13 13 0 0 0 1.172 8l.195.288c.335.48.83 1.12 1.465 1.755C4.121 11.332 5.881 12.5 8 12.5c.716 0 1.39-.133 2.02-.36l.77.772A7 7 0 0 1 8 13.5C3 13.5 0 8 0 8s.939-1.721 2.641-3.238l.708.709zm10.296 8.884-12-12 .708-.708 12 12z"/>
                        </svg>
                    }
                </button>
            }
            if input_value.clone().chars().count() as i32 > 0 {
                <button
                    {onclick}
                    class={classes!("mr-3")}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x-lg" viewBox="0 0 16 16">
                      <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8z"/>
                    </svg>
                </button>
            }
            { props.end_decorator.clone() }
        </div>
    }
}