use crate::basic_components::layout::Layout;
use crate::components::header::header::Header;
use crate::route::route::{switch, Route};
use crate::theme::size::Size;
use yew::prelude::*;
use yew_router::prelude::*;

mod basic_components;
mod theme;
mod event;
mod components;
mod views;
mod route;

#[function_component(App)]
fn app() -> Html {
    let header_height_num = Size::Md.get().1 as f64 * 1.5;
    let header_height = format!("{}{}{}", "h-[", header_height_num, "px]");

    html! {
        <BrowserRouter>
            <Layout>
                <header class={classes!(header_height, "z-[9999]")}>
                    <Header/>
                </header>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </Layout>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
