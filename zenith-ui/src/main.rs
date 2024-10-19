use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::header::header::Header;
use basic_components::context::theme_ctx::ThemeProvider;
use crate::components::container::Container;
use crate::theme::size::Size;
use crate::views::home_view::HomeView;

mod basic_components;
mod theme;
mod event;
mod components;
mod views;

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
        Route::Home => html! { <HomeView/> },
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
    let header_height_num = Size::Md.get().1 as f64 * 1.5;
    let header_height = format!("{}{}{}", "h-[", header_height_num, "px]");

    html! {
        <BrowserRouter>
            <ThemeProvider>
                <Container>
                    <div class="pl-10 pr-10 relative min-h-full">
                        <header class={classes!(header_height)}>
                            <Header/>
                        </header>
                        <main>
                            <Switch<Route> render={switch} />
                        </main>
                    </div>
                </Container>
            </ThemeProvider>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
