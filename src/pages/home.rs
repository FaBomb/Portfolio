use yew::prelude::*;
use yew_router::prelude::*;
use crate::{routing::AppRoute};

use js_bridge::{test_connect};

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    test_connect("yahh00o!");

    let onclick_callback = Callback::from(move |_| history.push(AppRoute::Blog));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button onclick={onclick_callback}>{ "Go Blog" }</button>
        </div>
    }
}