use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::header::Header;

mod basic_components;
mod theme;
mod event;
mod components;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/image")]
    Image,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <></> },
        Route::Blog => html! {
            <p>{"Blog"}</p>
        },
        Route::Image => html! {
            <p>{"Image"}</p>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="pl-10 pr-10 relative min-h-full">
                <Header/>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
