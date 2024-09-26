use web_sys::MouseEvent;
use yew::Callback;

pub fn on_click(onclick: Option<Callback<()>>, condition: Option<bool>) -> Callback<MouseEvent> {
    let on_click = {
        let onclick = onclick.clone();
        if onclick.is_some() {
            let onclick = onclick.unwrap().clone();

            Callback::from(move |_| {
                if condition.is_some() {
                    if condition.unwrap() {
                        onclick.emit(());
                    }
                } else {
                    onclick.emit(());
                }
            })
        } else { Callback::from(|_| ()) }
    };
    on_click
}