use yew::{classes, function_component, html, use_context, Html, Properties};
use crate::basic_components::context::theme_ctx::ThemeContext;
use crate::theme::default;

#[derive(Properties, Debug, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();

    let bg = default::Default::Theme.bg(theme_ctx.inner.as_str());

    html! {
        <div class={classes!("w-full", "h-full", bg, "font-mono", "antialiased")}>
            {props.children.clone()}
        </div>
    }
}