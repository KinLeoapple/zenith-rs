use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;
use crate::theme::color;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Theme {
    pub inner: String,
}

impl Reducible for Theme {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let storage = window().unwrap().local_storage().unwrap().unwrap();
        storage.set_item("theme", &*self.inner).unwrap();
        Theme { inner: action }.into()
    }
}

pub type ThemeContext = UseReducerHandle<Theme>;

#[derive(Properties, Debug, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let initial_theme = storage
        .get_item("theme")
        .unwrap_or_default()
        .unwrap_or(color::Theme::Dark.theme().to_string());

    let msg = use_reducer(|| Theme {
        inner: initial_theme,
    });

    html! {
        <ContextProvider<ThemeContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}