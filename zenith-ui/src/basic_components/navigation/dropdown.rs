use yew::{function_component, html, Html, Properties};
use crate::basic_components::data_display::list::List;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Properties, PartialEq)]
pub struct DropdownProp {
    #[prop_or_default]
    pub button: Html, // dropdown button
    #[prop_or(2)]
    pub list_gap: i32, // the gap of each item
    #[prop_or(true)]
    pub vertical: bool, // is vertical
    #[prop_or_default]
    pub divider: Html, // divider of the list
    #[prop_or_default]
    pub list_items: Vec<Html>, // the list of list items
}

#[function_component(Dropdown)]
pub fn dropdown(
    props: &DropdownProp,
) -> Html {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let id = format!("dropdown{}", rand_string);

    html! {
        <>
            <button data-dropdown-toggle={id.clone()} class="flex text-sm bg-gray-800 rounded-full md:me-0 focus:ring-4 focus:ring-gray-300 dark:focus:ring-gray-600" type="button">
                {props.button.clone()}
            </button>
            <div id={id.clone()} class="z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600">
                <List list_gap={props.list_gap} list_items={props.list_items.clone()} divider={props.divider.clone()}/>
            </div>
        </>
    }
}