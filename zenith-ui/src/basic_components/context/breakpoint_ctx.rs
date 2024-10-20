use crate::theme::window_size::WindowSize;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Breakpoint {
    pub inner: String,
}

impl Reducible for Breakpoint {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        if action.clone() != self.inner {
            Breakpoint { inner: action }.into()
        } else {
            self.into()
        }
    }
}

pub type BreakpointContext = UseReducerHandle<Breakpoint>;

#[derive(Properties, Debug, PartialEq)]
pub struct BreakpointProviderProp {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(BreakpointProvider)]
pub fn window_breakpoint_provider(props: &BreakpointProviderProp) -> Html {
    let window_size = WindowSize::new();
    let initial_breakpoint = window_size.breakpoint(window_size.get_window_size().0).to_string();

    let window_breakpoint = use_reducer(|| Breakpoint {
        inner: initial_breakpoint,
    });

    html! {
        <ContextProvider<BreakpointContext> context={window_breakpoint}>
            {props.children.clone()}
        </ContextProvider<BreakpointContext>>
    }
}