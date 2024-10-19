use yew::{html, Html};
use yew_router::Routable;
use crate::views::home_view::HomeView;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sign-up")]
    SignUp,
    #[at("/sign-in")]
    SignIn,
    #[at("/blog")]
    Blog,
    #[at("/image")]
    Image,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomeView/> },
        Route::SignUp => html! {<></>},
        Route::SignIn => html! {<></>},
        Route::Blog => html! {
            <p>{"Blog"}</p>
        },
        Route::Image => html! {
            <p>{"Image"}</p>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}