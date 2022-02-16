use crate::compornents::{footer::Footer, header::Header};
use crate::routing::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(AppRoute::Home));
    html! {
        <>
            <Header/>
            <h1>{ "404エラー" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
            <Footer/>
        </>
    }
}
