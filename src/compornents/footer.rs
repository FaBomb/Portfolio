use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <h2>{ "Footer" }</h2>
        </footer>
    }
}
