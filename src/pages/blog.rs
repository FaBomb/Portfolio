use crate::compornents::{footer::Footer, header::Header};
use crate::routing::AppRoute;
use js_bridge::{fetch_article_content_from_id, is_signed_in};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
use yew::virtual_dom::VNode;
use yew::{function_component, html, use_effect_with_deps, use_state};
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let history = use_history().unwrap();

    html! {
        <>
            <Header/>
            <h1>{ "Blog" }</h1>
            <Footer/>
        </>
    }
}
