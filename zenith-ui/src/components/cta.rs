use yew::{function_component, html, Html};
use crate::basic_components::data_display::typography::{Level, Typography};

#[function_component(CTA)]
pub fn cta() -> Html {
    html! {
        <>
            <Typography level={Level::H2} text={"Exploring"}/><br/>
            <Typography level={Level::H2} text={"Horizons of Thought."}/>
        </>
    }
}