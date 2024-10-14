use yew::{classes, function_component, html, use_state, Callback, Html, Properties};
use crate::event::on_click::on_click;

#[derive(Properties, PartialEq)]
pub struct ImageProp {
    #[prop_or_default]
    pub width: i32, // width of the image
    #[prop_or_default]
    pub height: i32, // height of the image
    #[prop_or(true)]
    pub rounded: bool, // border rounded
    #[prop_or_default]
    pub src: String, // the src of the image
    #[prop_or(None)]
    pub on_click: Option<Callback<()>>, // on click event
}

#[function_component(Image)]
pub fn image(
    props: &ImageProp,
) -> Html {
    let width = if props.width.clone() == 0 {
        "w-full".to_string()
    } else { format!("{}{}{}", "w-[", props.width, "px]") };
    let height = if props.height.clone() == 0 {
        "h-full".to_string()
    } else { format!("{}{}{}", "h-[", props.height, "px]") };
    let rounded = if props.rounded.clone() { Some("rounded-md") } else { None };
    let clickable = if props.on_click.is_some() { Some("cursor-pointer") } else { None };

    let src_handle = use_state(|| format!("{}", props.src.clone()));
    let src = (*src_handle).clone();

    let onclick = on_click(props.on_click.clone(), None);

    html! {
        <div {onclick} class={classes!(width, height, rounded, clickable, "overflow-hidden")}>
            <img class="object-cover w-full h-full"
                src={src.clone()}
                alt=""
              />
        </div>
    }
}