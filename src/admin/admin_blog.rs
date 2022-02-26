use crate::compornents::{footer::Footer, header::Header};
use crate::routing::{AdminBlogRoute, AppRoute};
use js_bridge::{fetch_article_contents, fetch_article_size, is_signed_in};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::virtual_dom::VNode;
use yew::{function_component, html, use_effect, use_effect_with_deps, use_state, Properties};
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
    pub page: String,
}
fn create_pagination_vnode(page_num: u8, current_page: u8) -> Vec<VNode> {
    let mut pagination_vnode: Vec<VNode> = Vec::new();
    for page in 1..page_num {
        let page_change = {
            move |_| {
                log::info!("{:?}", page);
                create_pagination_vnode(page_num, page);
            }
        };
        let li_tag;
        if page == current_page {
            li_tag = html! {
                <li style="color:red ">{page}</li>
            };
        } else {
            li_tag = html! {
                <li onclick={page_change}>{page}</li>
            };
        }
        pagination_vnode.push(li_tag);
    }
    pagination_vnode
}

struct CardsVnode {
    cards_vnode: Vec<VNode>,
}

#[function_component(AdminBlog)]
pub fn admin_blog(props: &RenderedAtProps) -> Html {
    let history = use_history().unwrap();
    let is_signed = use_state(|| false);
    let article_size = use_state(|| 1);
    let current_page = use_state(|| 1);
    let last_page_num = use_state(|| 1);
    let init_article: Vec<Article> = Vec::new();
    let article_contents = use_state(|| init_article);

    let limit_num = 3;

    {
        let is_signed = is_signed.clone();
        let history = history.clone();
        let last_page_num = last_page_num.clone();
        use_effect_with_deps(
            move |_| {
                let is_signed = is_signed.clone();
                spawn_local(async move {
                    // let result = is_signed_in("_").await.as_bool().unwrap();
                    // is_signed.set(result);
                    // if !result {
                    //     history.push(AppRoute::Admin);
                    // }

                    let fetch_page_size = fetch_article_size("blog".to_string())
                        .await
                        .as_f64()
                        .unwrap()
                        .round() as u8;
                    let page_size;
                    if fetch_page_size % limit_num != 0 {
                        page_size = fetch_page_size / limit_num + 2;
                    } else {
                        page_size = fetch_page_size / limit_num + 1;
                    }
                    last_page_num.set(page_size);
                    // pagination_vnode.set(create_pagination_vnode(page_size, *current_page));
                    log::info!("render {:?}", "!!");
                });
                || ()
            },
            (),
        );
    }

    pub async fn async_fetch_article(
        cards_vnode: Rc<RefCell<Vec<VNode>>>,
        current: u8,
        limit_num: u8,
    ) -> Result<Rc<RefCell<Vec<VNode>>>> {
        let article_contents_value = fetch_article_contents("blog".to_string(), current, limit_num)
            .await
            .as_string()
            .unwrap();
        let article_contents_vec: Result<Vec<Article>> =
            serde_json::from_str(&article_contents_value);
        for article_content in article_contents_vec.unwrap() {
            let history = use_history().unwrap();
            let go_view = Callback::once(move |_| {
                history.push(AppRoute::View {
                    id: article_content.id,
                })
            });
            let card = html! {
                <div class="card" onclick={go_view}>
                <img src={article_content.thumbnail}/>
                <time>{article_content.updated_at}</time>
                <h1>{article_content.title}</h1>
                </div>
            };
            cards_vnode.borrow_mut().push(card);
        }

        Ok(cards_vnode)
    }
    let cards_vnode: Rc<RefCell<Vec<VNode>>> = Rc::new(RefCell::new(Vec::new()));
    let current = current_page.clone();
    let state = async move { async_fetch_article(cards_vnode, *current, limit_num).await };
    let vnode = state;
    log::info!("outer {:?}", vnode);

    let current_page = current_page.clone();
    log::info!("outer {:?}", current_page);
    let mut pagination: Vec<VNode> = Vec::new();
    for page in 1..*last_page_num {
        let page_change = {
            let history = history.clone();
            move |_| {
                history.push(AdminBlogRoute::AdminBlog);
            }
        };
        let li_tag;
        let page_name = format!("{}{}", "#".to_string(), page.to_string());
        if page == *current_page {
            li_tag = html! {
                <li style="color:red ">{page}</li>
            };
        } else {
            li_tag = html! {
                <li onclick={page_change}>{page}</li>
            };
        }
        pagination.push(li_tag);
    }
    let pagination_vnode = pagination.clone();
    html! {
        <>
            <Header/>
            <h1>{ "Admin Blog" }</h1>
            // {cards_vnode.to_vec()}
            <ul class="pagenation">
                {pagination_vnode.to_vec()}
            </ul>
            <Footer/>
        </>
    }
}
