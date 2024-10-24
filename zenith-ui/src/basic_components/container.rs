use crate::basic_components::context::breakpoint_ctx::BreakpointContext;
use crate::basic_components::context::theme_ctx::ThemeContext;
use crate::theme::theme_value::ThemeValue;
use crate::theme::window_size::WindowSize;
use gloo::events::EventListener;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{window, Event, HtmlElement};
use yew::{classes, function_component, html, use_context, use_effect_with, use_node_ref, Callback, Html, Properties};
use crate::basic_components::scrollbar::Scrollbar;

#[wasm_bindgen(module = "/wasm_bindgen/change_bg.js")]
extern "C" {
    fn change_bg(class_name: &str);

    fn drop_bg(class_name: &str);
}

#[derive(Properties, Debug, PartialEq)]
pub struct ContainerProp {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Container)]
pub fn container(props: &ContainerProp) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();

    let breakpoint_ctx = use_context::<BreakpointContext>().unwrap();

    let bg = ThemeValue::Theme.bg(theme_ctx.inner.as_str());

    let div_node_ref = use_node_ref();

    // Change breakpoint
    use_effect_with(
        div_node_ref.clone(),
        {
            let div_node_ref = div_node_ref.clone();
            let window_breakpoint_ctx = breakpoint_ctx.clone();

            move |_| {
                let mut resize_listener = None;

                if let Some(_element) = div_node_ref.cast::<HtmlElement>() {
                    // Create onresize callback
                    let onresize = Callback::from(move |_: Event| {
                        let window_size = WindowSize::new();
                        let window_breakpoint = window_size.breakpoint(window_size.get_window_size().0).to_string();
                        window_breakpoint_ctx.dispatch(window_breakpoint);
                    });

                    // Create a Closure from a Box<dyn Fn> - this has to be 'static
                    let listener = EventListener::new(
                        &window().unwrap(),
                        "resize",
                        move |e| onresize.emit(e.clone())
                    );

                    resize_listener = Some(listener);
                }

                move || drop(resize_listener)
            }
        }
    );

    // Change background color
    use_effect_with(
        bg,
        {
            move |_| {
                match theme_ctx.inner.as_str() {
                    "dark" => drop_bg(ThemeValue::Theme.bg("light")),
                    "light" => drop_bg(ThemeValue::Theme.bg("dark")),
                    _ => {}
                };
                change_bg(bg);
            }
        }
    );

    html! {
        <Scrollbar layer_top={true}>
            <div id="root" ref={div_node_ref} class={classes!("w-full", "h-full", "pl-10", "pr-10", bg, "font-mono", "antialiased")}>
                {props.children.clone()}
            </div>
        </Scrollbar>
    }
}