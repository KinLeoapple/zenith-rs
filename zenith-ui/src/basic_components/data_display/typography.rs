use crate::basic_components::context::theme_ctx::ThemeContext;
use crate::event::on_click::on_click;
use crate::theme::default;
use yew::{classes, function_component, html, use_context, Callback, Html, Properties};

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
    #[prop_or_default]
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
    let theme_ctx = use_context::<ThemeContext>().unwrap();

    let shadow = if props.shadow.clone() { Some("shadow-md") } else { None };
    let clickable = if props.on_click.is_some() { Some("cursor-pointer") } else { None };
    let bg_color = if props.bg_color.is_some() { Some(format!("{}{}{}", "bg-[", props.bg_color.clone().unwrap(), "]")) } else { None };
    let text_color = format!("{}{}{}", "text-[", if props.text_color.clone().chars().count() <= 0 {
        default::Default::Theme.text_color(theme_ctx.inner.as_str())
    } else {
        props.text_color.as_ref()
    }, "]");
    let font_size = format!("{}{}", "text-",
                            match props.level.clone() {
                                Level::H1 => "7xl",
                                Level::H2 => "6xl",
                                Level::H3 => "5xl",
                                Level::H4 => "4xl",
                                Level::TitleLg => "3xl",
                                Level::TitleMd => "2xl",
                                Level::TitleSm => "xl",
                                Level::BodyLg => "lg",
                                Level::BodyMd => "base",
                                Level::BodySm => "sm",
                                Level::BodyXs => "xs",
                            });

    let onclick = on_click(props.on_click.clone(), None);

    html! {
        <div
            {onclick}
            class={classes!(font_size.clone(), "pl-2", "pr-2", clickable, "relative", "inline-flex", "items-center", "justify-center", "shrink-0", "transition-colors", "duration-100", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "font-bold", bg_color, text_color, "rounded", shadow)}>
            { props.start_decorator.clone() }
            <div
                class={classes!("relative", "inline-flex", "items-center", "justify-center", "transition-colors", "duration-100", "transition-colors", "outline-none", "disabled:pointer-events-none", "disabled:opacity-50", "font-bold")}>
                <div class="origin-left">{format!("{}", props.text)}</div>
            </div>
            { props.end_decorator.clone() }
        </div>
    }
}