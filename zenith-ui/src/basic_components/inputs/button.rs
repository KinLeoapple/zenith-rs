use crate::basic_components::context::theme_ctx::ThemeContext;
use crate::basic_components::data_display::typography::Typography;
use crate::basic_components::icon::Icon;
use crate::event::on_click::on_click;
use crate::theme::size::Size;
use crate::theme::theme_value::ThemeValue;
use yew::{classes, function_component, html, use_context, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProp {
    #[prop_or(Size::Md)]
    pub size: Size, // size
    #[prop_or_default]
    pub text: String, // text
    #[prop_or_default]
    pub icon: Html, // icon
    #[prop_or(true)]
    pub rounded: bool, // border rounded
    #[prop_or(true)]
    pub shadow: bool, // show box shadow
    #[prop_or(false)]
    pub is_decorator: bool, // is decorator
    #[prop_or(false)]
    pub loading: bool, // is loading
    #[prop_or(false)]
    pub disabled: bool, // disabled
    #[prop_or_default]
    pub start_decorator: Html, // start decorator
    #[prop_or_default]
    pub end_decorator: Html, // end decorator
    #[prop_or(None)]
    pub on_click: Option<Callback<()>>, // on click event
}

#[function_component(Button)]
pub fn button(
    props: &ButtonProp
) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();

    let width = format!("{}{}{}", "min-w-[", props.size.get().0 as f64 * 1.2, "px]");
    let height = format!("{}{}{}", "h-[", match props.is_decorator {
        true => props.size.get().1 - 10,
        false => props.size.get().1
    }, "px]");
    let rounded = if props.rounded.clone() { Some("rounded-full") } else { None };
    let margin = match props.is_decorator {
        true => Some("mx-[3px]"),
        false => None
    };
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };
    let clickable = if props.on_click.is_some() { Some("cursor-pointer") } else { None };

    let bg_color = format!("{}{}{}", "bg-[", ThemeValue::Theme.button_bg_color(theme_ctx.inner.as_str()), "]");
    let bg_hover_color = format!("{}{}{}", "hover:bg-[", ThemeValue::Theme.button_hover_bg_color(theme_ctx.inner.as_str()), "]");
    let text_color = format!("{}{}{}", "text-[", ThemeValue::Theme.text_color(theme_ctx.inner.as_str()), "]");

    let onclick = on_click(props.on_click.clone(), Some(!props.loading.clone()));

    html! {
        <button
            disabled={props.disabled.clone()}
            {onclick}
            class={classes!("pl-5", "pr-5", "select-none", "w-auto", clickable, "relative", "inline-flex", "items-center", "justify-center", "shrink-0", "transition-colors", "duration-100", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "font-bold", bg_color, bg_hover_color, text_color, width, height, rounded, margin, shadow)}>
            <div class={classes!("select-none", clickable, "relative", "inline-flex", "items-center", "justify-center", "shrink-0")}>
                { props.start_decorator.clone() }
            </div>
            <div
                class={classes!("relative", "inline-flex", "items-center", "justify-center", "transition-colors", "duration-100", "transition-colors", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "font-bold")}>
                if props.loading.clone() {
                    <Icon svg={
                        html!(<svg aria-hidden="true" role="status"
                        class={classes!("inline", "w-5", "h-5", "animate-spin")} viewBox="0 0 100 101" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                        <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="#E5E7EB"/>
                        <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentColor"/>
                        </svg>)
                    } />
                } else {
                    if props.text.clone().chars().count() > 0 {
                        <Typography text={format!("{}", props.text)}/>
                    } else {
                        <Icon svg={props.icon.clone()}/>
                    }
                }
            </div>
            <div class={classes!("select-none", clickable, "relative", "inline-flex", "items-center", "justify-center", "shrink-0")}>
                { props.end_decorator.clone() }
            </div>
        </button>
    }
}