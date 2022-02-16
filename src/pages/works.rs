use crate::compornents::{footer::Footer, header::Header};
use crate::routing::AppRoute;
use yew::{function_component, html, Callback};
use yew_router::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(AppRoute::Home));
    html! {
        <>
            <Header/>
            <h1>{ "Works" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
            <Footer/>
        </>
    }
}
