use crate::compornents::{footer::Footer, header::Header};
use crate::routing::BlogRoute;
use yew::{function_component, html, Callback};
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| {
        history.push(BlogRoute::Blog {
            page: "1".to_string(),
        })
    });

    html! {
        <>
            <Header/>
            <h1>{ "Home" }</h1>
            <button onclick={onclick_callback}>{ "Go Blog" }</button>
            <Footer/>
        </>
    }
}
