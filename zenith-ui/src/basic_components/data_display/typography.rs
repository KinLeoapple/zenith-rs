use crate::event::on_click::on_click;
use crate::theme::color::Common;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(PartialEq, Clone, Debug)]
pub enum Level {
    H1,
    H2,
    H3,
    H4,
    TitleLg,
    TitleMd,
    TitleSm,
    BodyLg,
    BodyMd,
    BodySm,
    BodyXs,
}

#[derive(Properties, PartialEq)]
pub struct TypographyProp {
    #[prop_or(Level::BodyMd)]
    pub level: Level, // text level
    #[prop_or_default]
    pub text: String, // text
    #[prop_or(Common::White.common().to_string())]
    pub text_color: String, // text color
    #[prop_or_default]
    pub bg_color: Option<String>, // background color
    #[prop_or(false)]
    pub shadow: bool, // show box shadow
    #[prop_or_default]
    pub start_decorator: Html, // start decorator
    #[prop_or_default]
    pub end_decorator: Html, // end decorator
    #[prop_or(None)]
    pub on_click: Option<Callback<()>>, // on click event
}

#[function_component(Typography)]
pub fn typography(
    props: &TypographyProp,
) -> Html {
    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };
    let clickable = if props.on_click.is_some() { Some("cursor-pointer") } else { None };
    let bg_color = if props.bg_color.is_some() { Some(format!("{}{}{}", "bg-[", props.bg_color.clone().unwrap(), "]")) } else { None };
    let text_color = format!("{}{}{}", "text-[", props.text_color.clone(), "]");
    let font_size = if props.level.clone() != Level::H1
        || props.level.clone() != Level::H2
        || props.level.clone() != Level::H3
        || props.level.clone() != Level::H4 {
        Some(format!("{}{}", "text-",
                     match props.level.clone() {
                         Level::TitleLg => "3xl",
                         Level::TitleMd => "2xl",
                         Level::TitleSm => "xl",
                         Level::BodyLg => "lg",
                         Level::BodyMd => "base",
                         Level::BodySm => "sm",
                         Level::BodyXs => "xs",
                         _ => ""
                     }))
    } else { None };

    let onclick = on_click(props.on_click.clone(), None);

    html! {
        <div
            {onclick}
            class={classes!(font_size.clone(), "pl-2", "pr-2", "select-none", clickable, "relative", "inline-flex", "items-center", "justify-center", "shrink-0", "transition-colors", "duration-100", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "font-bold", bg_color, text_color, "rounded", shadow)}>
            { props.start_decorator.clone() }
            <div
                class={classes!("relative", "inline-flex", "items-center", "justify-center", "transition-colors", "duration-100", "transition-colors", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "font-bold")}>
                if props.level.clone() != Level::H1
                    || props.level.clone() != Level::H2
                    || props.level.clone() != Level::H3
                    || props.level.clone() != Level::H4 {
                    <div class="origin-left">{format!("{}", props.text)}</div>
                }
                if props.level.clone() == Level::H1 {
                    <h1 class="origin-left">{format!("{}", props.text)}</h1>
                }
                if props.level.clone() == Level::H2 {
                    <h2 class="origin-left">{format!("{}", props.text)}</h2>
                }
                if props.level.clone() == Level::H3 {
                    <h3 class="origin-left">{format!("{}", props.text)}</h3>
                }
                if props.level.clone() == Level::H4 {
                    <h4 class="origin-left">{format!("{}", props.text)}</h4>
                }
            </div>
            { props.end_decorator.clone() }
        </div>
    }
}