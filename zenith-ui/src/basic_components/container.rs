use crate::basic_components::context::theme_ctx::ThemeContext;
use crate::theme::theme_value::ThemeValue;
use yew::{classes, function_component, html, use_context, Html, Properties};

#[derive(Properties, Debug, PartialEq)]
pub struct ContainerProp {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Container)]
pub fn container(props: &ContainerProp) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();

    let bg = ThemeValue::Theme.bg(theme_ctx.inner.as_str());

    html! {
        <div class={classes!("w-full", "min-h-full", "pl-10", "pr-10", "relative", bg, "font-mono", "antialiased")}>
            {props.children.clone()}
        </div>
    }
}