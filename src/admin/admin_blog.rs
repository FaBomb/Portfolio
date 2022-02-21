use crate::compornents::{footer::Footer, header::Header};
use crate::routing::AppRoute;
use js_bridge::{fetch_article_contents, fetch_article_size, is_signed_in};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
use yew::virtual_dom::VNode;
use yew::{function_component, html, use_effect_with_deps, use_state};
use yew_router::prelude::*;

// fn fetch_cards(ids: Vec<String>) -> Vec<VNode> {
//     let mut cards = Vec::new();
//     for card_id in ids {
//         let pulldown_option = html! {
//             <Card id={card_id}/>
//         };
//         cards.push(pulldown_option);
//     }
//     cards
// }

#[derive(Serialize, Deserialize, Debug)]
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

#[function_component(AdminBlog)]
pub fn admin_blog() -> Html {
    let history = use_history().unwrap();
    let is_signed = use_state(|| false);
    let article_size = use_state(|| 1);
    let current_page = use_state(|| 1);
    let last_page_num = use_state(|| 1);
    let init_page: Vec<VNode> = Vec::new();
    let pagination_vnode = use_state(|| init_page);
    let init_card: Vec<VNode> = Vec::new();
    let cards = use_state(|| init_card);

    let limit_num = 3;

    {
        let is_signed = is_signed.clone();
        let current_page = current_page.clone();
        let last_page_num = last_page_num.clone();
        let pagination_vnode = pagination_vnode.clone();
        use_effect_with_deps(
            move |_| {
                let current_page = current_page.clone();
                let is_signed = is_signed.clone();
                spawn_local(async move {
                    let result = is_signed_in("_").await.as_bool().unwrap();
                    is_signed.set(result);
                    if !result {
                        history.push(AppRoute::Admin);
                    }

                    let article_contents_value =
                        fetch_article_contents("blog".to_string(), *current_page, limit_num)
                            .await
                            .as_string()
                            .unwrap();
                    let article_contents_vec: Result<Vec<Article>> =
                        serde_json::from_str(&article_contents_value);
                    log::info!("vec {:?}", article_contents_vec.unwrap());

                    let page_size = fetch_article_size("blog".to_string())
                        .await
                        .as_f64()
                        .unwrap()
                        .round() as u8;
                    let fetch_page_num;
                    if page_size % limit_num != 0 {
                        fetch_page_num = page_size / limit_num + 2;
                    } else {
                        fetch_page_num = page_size / limit_num + 1;
                    }
                    last_page_num.set(fetch_page_num);
                    let mut page_buttons: Vec<VNode> = Vec::new();
                    for page in 1..fetch_page_num {
                        let page_change = {
                            let current_page = current_page.clone();
                            move |_| {
                                current_page.set(page);
                            }
                        };
                        let li_tag;
                        if page == *current_page {
                            li_tag = html! {
                                <li style="color:red ">{page}</li>
                            };
                        } else {
                            li_tag = html! {
                                <li onclick={page_change}>{page}</li>
                            };
                        }
                        page_buttons.push(li_tag);
                    }
                    pagination_vnode.set(page_buttons);
                });
                || ()
            },
            (),
        );
    }
    html! {
        <>
            <Header/>
            <h1>{ "Admin Blog" }</h1>
            // {card_vnode.to_vec()}
            <ul class="pagenation">
                {pagination_vnode.to_vec()}
            </ul>
            <Footer/>
        </>
    }
}
