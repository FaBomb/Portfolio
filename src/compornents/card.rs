use crate::routing::AppRoute;
use js_bridge::fetch_article_contents;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
use yew::virtual_dom::VNode;
use yew::{function_component, html, use_effect_with_deps, use_state, Properties};
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Article {
    id: String,
    content: String,
    tags: Vec<String>,
    category: String,
    released: bool,
    title: String,
    thumbnail: String,
    images: Vec<String>,
    updated_at: String,
}

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub current_page: u8,
    pub limit_num: u8,
    pub article_type: String,
}

#[function_component(Card)]
pub fn card(props: &RenderedAtProps) -> Html {
    let history = use_history().unwrap();
    let init_card_vnode: Vec<VNode> = Vec::new();
    let card_vnode = use_state(|| init_card_vnode);
    let current_page = use_state(|| props.current_page);
    let article_type = props.article_type.clone();

    {
        let card_vnode = card_vnode.clone();
        let props_current_page = props.current_page.clone();
        let limit_num = props.limit_num.clone();
        let history = history.clone();
        let article_type = article_type.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let article_contents_value =
                        fetch_article_contents(article_type, props_current_page, limit_num)
                            .await
                            .as_string()
                            .unwrap();
                    let article_contents_result: Result<Vec<Article>> =
                        serde_json::from_str(&article_contents_value);
                    let mut vnode: Vec<VNode> = Vec::new();
                    for article_content in article_contents_result.unwrap() {
                        let article_id = article_content.id;
                        let go_view = {
                            let history = history.clone();
                            let article_id = article_id.clone();
                            move |_| {
                                let article_id = article_id.clone();
                                history.push(AppRoute::View { id: article_id });
                            }
                        };
                        let card = html! {
                            <div class="card" onclick={go_view}>
                                <img src={article_content.thumbnail}/>
                                <time>{article_content.updated_at}</time>
                                <h1>{article_content.title}</h1>
                            </div>
                        };
                        vnode.push(card);
                    }
                    card_vnode.set(vnode);
                });
                || ()
            },
            (),
        );
    }

    let change_article = {
        let props_current_page = props.current_page.clone();
        let limit_num = props.limit_num.clone();
        let card_vnode = card_vnode.clone();
        let current_page = current_page.clone();
        let article_type = article_type.clone();
        move |_| {
            spawn_local(async move {
                let article_contents_value =
                    fetch_article_contents(article_type, props_current_page, limit_num)
                        .await
                        .as_string()
                        .unwrap();
                let article_contents_result: Result<Vec<Article>> =
                    serde_json::from_str(&article_contents_value);
                let mut vnode: Vec<VNode> = Vec::new();
                for article_content in article_contents_result.unwrap() {
                    let article_id = article_content.id;
                    let go_view = {
                        let history = history.clone();
                        let article_id = article_id.clone();
                        move |_| {
                            let article_id = article_id.clone();
                            history.push(AppRoute::View { id: article_id });
                        }
                    };
                    let card = html! {
                        <div class="card" onclick={go_view}>
                            <img src={article_content.thumbnail}/>
                            <time>{article_content.updated_at}</time>
                            <h1>{article_content.title}</h1>
                        </div>
                    };
                    vnode.push(card);
                }
                card_vnode.set(vnode);
                current_page.set(props_current_page);
            });
        }
    };

    if props.current_page != *current_page {
        change_article("");
    }
    let new_card_vnode: Vec<VNode> = card_vnode.to_vec();
    html! {
        <>
            {new_card_vnode}
        </>
    }
}
