use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routing::AppRoute};

#[function_component(Blog)]
pub fn blog() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(AppRoute::Home));

    
    
    html! {
        <div>
            <h1>{ "fBlog" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}