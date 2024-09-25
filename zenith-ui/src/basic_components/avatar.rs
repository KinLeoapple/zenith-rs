use yew::{classes, function_component, html, use_state, Callback, Html, Properties};
use crate::theme::color::{Color, Common};
use crate::theme::size::Size;

#[derive(Properties, PartialEq)]
pub struct AvatarProp {
    #[prop_or(Size::Md)]
    pub size: Size, // size
    #[prop_or_default]
    pub src: String, // the src of the image
    #[prop_or_default]
    pub alt: String, // display text
    #[prop_or(true)]
    pub circle: bool, // border circle
    #[prop_or(true)]
    pub shadow: bool, // show box shadow
    #[prop_or(false)]
    pub is_decorator: bool, // is decorator
    #[prop_or(false)]
    pub dot_indicator: bool, // show dot indicator
    #[prop_or(Color::_500.success().to_string())]
    pub indicator_color: String,
    #[prop_or_default]
    pub on_click: Callback<()>, // on click event
}

#[function_component(Avatar)]
pub fn avatar(
    props: &AvatarProp
) -> Html {
    let width = format!("{}{}{}", "w-[", match props.is_decorator {
        true => props.size.get().1 - 10,
        false => props.size.get().1
    }, "px]");
    let height = format!("{}{}{}", "h-[", match props.is_decorator {
        true => props.size.get().1 - 10,
        false => props.size.get().1
    }, "px]");
    let circle = if props.circle.clone() { Some("rounded-full") } else { None };
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };
    let margin = match props.is_decorator {
        true => Some("mx-[3px]"),
        false => None
    };

    let bg_color = format!("{}{}{}", "bg-[", Color::_500.neutral(), "]");
    let icon_color = format!("{}{}{}", "bg-[", Color::_400.neutral(), "]");
    let text_color = format!("{}{}{}", "text-[", Common::White.common(), "]");
    let indicator_color = format!("{}{}{}", "text-[", props.indicator_color.clone(), "]");

    let alt_handle = use_state(|| format!("{}", props.alt.clone()));
    let alt = (*alt_handle).clone();

    let src_handle = use_state(|| format!("{}", props.src.clone()));
    let src = (*src_handle).clone();

    let onclick = {
        let on_click = props.on_click.clone();

        Callback::from(move |_| {
            on_click.emit(());
        })
    };

    html! {
        <div class={classes!("select-none", "relative", bg_color, text_color, width.clone(), height.clone(), circle.clone(), shadow, margin)}>
            <div
                {onclick}
                class={classes!("relative", "inline-flex", "items-center", "justify-center", width.clone(), height.clone(), circle.clone(), "overflow-hidden")}>
                    if src.chars().count() > 0 || alt.chars().count() > 0 {
                        if src.chars().count() > 0 {
                            <img class={classes!(width.clone(), height.clone(), circle.clone())}
                                src={src.clone()} alt="" />
                        } else {
                            <span class={classes!("text-2xl", "font-bold")}>
                                {props.alt.clone()}
                            </span>
                        }
                    } else {
                        <svg
                            class={classes!("absolute", icon_color, "-left-1")} fill="currentColor"
                            height={format!("{}", props.size.get().1 + 8)}
                            width={format!("{}", props.size.get().1 + 8)}
                            viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path></svg>
                }
            </div>
            if props.dot_indicator.clone() {
                <span
                    class={classes!(indicator_color, "bottom-0", "left-7", "absolute", "w-3.5", "h-3.5", "border-2", "rounded-full")}></span>
            }
        </div>
    }
}