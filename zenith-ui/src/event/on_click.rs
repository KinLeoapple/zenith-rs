use web_sys::MouseEvent;
use yew::Callback;

pub fn on_click(onclick: Option<Callback<()>>) -> Callback<MouseEvent> {
    let on_click = {
        let onclick = onclick.clone();
        if onclick.is_some() {
            let onclick = onclick.unwrap().clone();

            Callback::from(move |_| {
                onclick.emit(());
            })
        } else { Callback::from(|_| ()) }
    };
    on_click
}