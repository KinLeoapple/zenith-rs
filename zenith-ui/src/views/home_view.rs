use yew::{classes, function_component, html, Html};
use crate::components::cta::CTA;

#[function_component(HomeView)]
pub fn home_view() -> Html {
    html! {
        <div class={classes!("pt-20")}>
            <CTA/>
        </div>
    }
}