use yew::{function_component, html, Html, Properties};
use crate::basic_components::container::Container;
use crate::basic_components::context::theme_ctx::ThemeProvider;

#[derive(Properties, Debug, PartialEq)]
pub struct LayoutProp {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProp) -> Html {
    html! {
        <ThemeProvider>
            <Container>
                {props.children.clone()}
            </Container>
        </ThemeProvider>
    }
}