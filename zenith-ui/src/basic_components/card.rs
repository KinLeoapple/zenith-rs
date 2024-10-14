use yew::{classes, function_component, html, use_state, Html, Properties};
use crate::basic_components::image::Image;
use crate::theme::color::{Color, Text};
use crate::theme::size::Size;

#[derive(Properties, PartialEq)]
pub struct CardProp {
    #[prop_or(Size::Md)]
    pub size: Size,
    #[prop_or(true)]
    pub rounded: bool, // border rounded
    #[prop_or(true)]
    pub border: bool, // show border
    #[prop_or(true)]
    pub shadow: bool, // show box shadow
    #[prop_or(true)]
    pub vertical: bool, // is vertical
    #[prop_or_default]
    pub title: String, // title of the card
    #[prop_or_default]
    pub image_src: String, // the src of cover image
    #[prop_or_default]
    pub content: Html, // content of the card
    #[prop_or_default]
    pub actions: Vec<Html>, // action of the card (e.g. button)
}

#[function_component(Card)]
pub fn card(
    props: &CardProp
) -> Html {
    let width = format!("{}{}{}", "w-[", props.size.get().0 as f64 *
        match props.vertical.clone() {
        true => 5.0,
        false => 5.5
    }, "px]");
    let height = format!("{}", "h-auto");
    let rounded = if props.rounded.clone() { Some("rounded-md") } else { None };
    let border = if props.border.clone() { Some("border") } else { None };
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };
    let vertical = match props.vertical.clone() {
        true => "flex-col",
        false => "flex-row",
    };

    let bg_color = format!("{}{}{}", "bg-[", Color::_900.primary(), "]");
    let text_color = format!("{}{}{}", "text-[", Text::Primary.dark(), "]");
    let content_color = format!("{}{}{}", "text-[", Color::_400.neutral(), "]");
    let border_color = format!("{}{}{}", "border-[", Color::_500.primary(), "]");
    let padding = match props.vertical.clone() {
        true => Some("pl-5 pt-5 pr-5"),
        false => None,
    };
    let image_width = match props.vertical.clone() {
        true => None,
        false => Some("w-1/4")
    };

    let image_src_handle = use_state(|| format!("{}", props.image_src.clone()));
    let image_src = (*image_src_handle).clone();

    let actions_handle = use_state(|| props.actions.clone());
    let actions = (*actions_handle).clone();

    html! {
        <div class={classes!("flex", vertical.clone(), width.clone(), height.clone(), bg_color.clone(), border.clone(), border_color.clone(), rounded.clone(), shadow.clone(), "overflow-hidden")}>
            <div class={classes!(padding.clone(), image_width.clone(), "shrink-0")}>
                <Image rounded={props.vertical.clone()} src={image_src.clone()}/>
            </div>
            <div class="w-full p-5 flex flex-col items-center">
                <h5 class={classes!("w-full", "mb-2", "text-2xl", "font-bold", "tracking-tight", text_color.clone())}>{props.title.clone()}</h5>
                <div class={classes!("w-full", "mb-3", content_color)}>{props.content.clone()}</div>
                <div class={classes!("w-full", "inline-flex", "items-center", "justify-between")}>
                    {actions.iter().map(|(html)| {
                        html! {
                            <>
                                {html.clone()}
                            </>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </div>
    }
}