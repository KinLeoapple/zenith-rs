use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;
use yew::{classes, function_component, html, use_effect_with, use_node_ref, Html, Properties};

#[wasm_bindgen(module = "/wasm_bindgen/scrollbar.js")]
extern "C" {
    fn scroll(el: HtmlElement, always_on_top: bool);
}

#[derive(Properties, Debug, PartialEq)]
pub struct ScrollbarProp {
    #[prop_or_default]
    pub children: Html,
    #[prop_or(false)]
    pub layer_top: bool,
}

#[function_component(Scrollbar)]
pub fn scrollbar(props: &ScrollbarProp) -> Html {
    let scroll_node_ref = use_node_ref();

    use_effect_with(
        scroll_node_ref.clone(),
        {
            let scroll_node_ref = scroll_node_ref.clone();
            let layer_top = props.layer_top.clone();

            move |_| {
                if let Some(element) = scroll_node_ref.cast::<HtmlElement>() {
                    scroll(element.clone(), layer_top);
                }
            }
        }
    );

    html! {
        <div ref={scroll_node_ref} class={classes!("relative", "h-full", "overflow-hidden")}>
            {props.children.clone()}
        </div>
    }
}