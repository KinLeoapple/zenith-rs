use crate::theme::window_size::WindowSize;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WindowBreakpoint {
    pub inner: String,
}

impl Reducible for WindowBreakpoint {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        WindowBreakpoint { inner: action }.into()
    }
}

pub type WindowBreakpointContext = UseReducerHandle<WindowBreakpoint>;

#[derive(Properties, Debug, PartialEq)]
pub struct WindowBreakpointProviderProp {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(WindowBreakpointProvider)]
pub fn window_breakpoint_provider(props: &WindowBreakpointProviderProp) -> Html {
    let window_size = WindowSize::new();
    let initial_window_breakpoint = window_size.breakpoint(window_size.get_window_size().0).to_string();

    let window_breakpoint = use_reducer(|| WindowBreakpoint {
        inner: initial_window_breakpoint,
    });

    html! {
        <ContextProvider<WindowBreakpointContext> context={window_breakpoint}>
            {props.children.clone()}
        </ContextProvider<WindowBreakpointContext>>
    }
}