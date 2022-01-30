use yew::prelude::*;
use yew_router::prelude::*;
use crate::{routing::AppRoute};

#[function_component(NotFound)]
pub fn notfound() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(AppRoute::Home));
    html! {
        <div>
            <h1>{ "404エラー" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}