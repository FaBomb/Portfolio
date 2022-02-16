use crate::compornents::{card::Card, footer::Footer, header::Header};
use crate::routing::AppRoute;
use js_bridge::{fetch_article_id, is_signed_in};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
use yew::virtual_dom::VNode;
use yew::{function_component, html, use_effect_with_deps, use_state};
use yew_router::prelude::*;

fn fetch_cards(ids: Vec<String>) -> Vec<VNode> {
    let mut cards = Vec::new();
    for card_id in ids {
        let pulldown_option = html! {
            <Card id={card_id}/>
        };
        cards.push(pulldown_option);
    }
    cards
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let history = use_history().unwrap();
    let article_ids = use_state(Vec::new);

    let card_vnode = fetch_cards(article_ids.to_vec());

    {
        let article_ids = article_ids.clone();
        use_effect_with_deps(
            move |_| {
                let article_ids = article_ids.clone();
                spawn_local(async move {
                    let article_ids_value =
                        fetch_article_id("blog".to_string(), "510riim4hXzhywUdBcFV".to_string())
                            .await
                            .as_string()
                            .unwrap();
                    let article_ids_vec: Result<Vec<String>> =
                        serde_json::from_str(&article_ids_value);
                    article_ids.set(article_ids_vec.unwrap());
                });
                || ()
            },
            (),
        );
    }
    html! {
        <>
            <Header/>
            <h1>{ "Blog" }</h1>
            {card_vnode}
            <Footer/>
        </>
    }
}
