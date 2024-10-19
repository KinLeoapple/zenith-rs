use crate::basic_components::icon::Icon;
use crate::basic_components::inputs::button::Button;
use crate::components::home::cta::CTA;
use yew::{classes, function_component, html, Callback, Html};
use yew_router::hooks::use_navigator;
use crate::Route;

#[function_component(HomeView)]
pub fn home_view() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.replace(&Route::SignIn));

    html! {
        <div class={classes!("pt-28")}>
            <div class={classes!("select-none", "pb-5")}>
                <CTA/>
            </div>
            <Button on_click={onclick} text={"Sign up"} end_decorator={
                html!(
                    <Icon svg={
                        html!(
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-up-right" viewBox="0 0 16 16">
                              <path fill-rule="evenodd" d="M14 2.5a.5.5 0 0 0-.5-.5h-6a.5.5 0 0 0 0 1h4.793L2.146 13.146a.5.5 0 0 0 .708.708L13 3.707V8.5a.5.5 0 0 0 1 0z"/>
                            </svg>
                        )
                    }/>
                )
            }/>
        </div>
    }
}