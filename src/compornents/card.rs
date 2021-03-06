use crate::routing::{AdminBlogRoute, AdminWorkRoute, ViewBlogRoute, ViewWorkRoute};
use js_bridge::{
    del_content, fetch_article_contents, fetch_query_contents, is_signed_in, update_released,
};
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
    pub is_signed: bool,
    pub query_content: String,
}

#[function_component(Card)]
pub fn card(props: &RenderedAtProps) -> Html {
    let rerender = use_state(|| false);
    let history = use_history().unwrap();
    let init_card_vnode: Vec<VNode> = Vec::new();
    let card_vnode = use_state(|| init_card_vnode);
    let current_page = use_state(|| props.current_page);
    let article_type = props.article_type.clone();
    let is_signed = props.is_signed.clone();
    let query_content = props.query_content.clone();

    {
        let card_vnode = card_vnode.clone();
        let rerender = rerender.clone();
        let props_current_page = props.current_page.clone();
        let limit_num = props.limit_num.clone();
        let history = history.clone();
        let article_type = article_type.clone();
        let query_content = query_content.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let is_signed_result = is_signed_in("_").await.as_bool().unwrap();
                    let article_contents_value;
                    if article_type == "tag" {
                        article_contents_value = fetch_query_contents(
                            article_type.clone(),
                            query_content,
                            props_current_page,
                            limit_num,
                        )
                        .await
                        .as_string();
                    } else {
                        article_contents_value = fetch_article_contents(
                            article_type.clone(),
                            props_current_page,
                            limit_num,
                            is_signed_result,
                        )
                        .await
                        .as_string();
                    }
                    let article_contents_value = match article_contents_value {
                        Some(article_content) => article_content,
                        None => "".to_string(),
                    };
                    let article_contents_result: Result<Vec<Article>> =
                        serde_json::from_str(&article_contents_value);

                    let article_contents_result = match article_contents_result {
                        Ok(article_contents) => article_contents,
                        Err(_) => vec![],
                    };
                    let mut vnode: Vec<VNode> = Vec::new();

                    for article_content in article_contents_result {
                        let article_id = article_content.id.clone();
                        let article_content_string =
                            serde_json::to_string(&article_content.clone()).unwrap();
                        let go_view = {
                            let history = history.clone();
                            let article_id = article_id.clone();
                            let article_type = article_type.clone();
                            move |_| {
                                let article_type = article_type.clone();
                                let article_id = article_id.clone();
                                if article_type == "blog" || article_type == "tag" {
                                    history.push(ViewBlogRoute::View { id: article_id });
                                } else if article_type == "works" {
                                    history.push(ViewWorkRoute::View { id: article_id });
                                }
                            }
                        };
                        let edit_article = {
                            let history = history.clone();
                            let article_id = article_id.clone();
                            move |_| {
                                let article_id = article_id.clone();
                                let path_name = history.location().pathname();
                                let path_name_vec: Vec<&str> = path_name.split('/').collect();
                                let some_path_name = path_name_vec.get(1);
                                let current_path = match some_path_name {
                                    Some(path) => path,
                                    None => "",
                                };
                                if current_path == "blog" {
                                    history
                                        .push(AdminBlogRoute::AdminArticleEdit { id: article_id });
                                } else if current_path == "works" {
                                    history
                                        .push(AdminWorkRoute::AdminArticleEdit { id: article_id });
                                }
                            }
                        };
                        let del_article = {
                            let history = history.clone();
                            let article_id = article_id.clone();
                            let rerender = rerender.clone();
                            move |_| {
                                let article_id = article_id.clone();
                                let path_name = history.location().pathname();
                                let rerender = rerender.clone();
                                spawn_local(async move {
                                    let path_name_vec: Vec<&str> = path_name.split('/').collect();
                                    let some_path_name = path_name_vec.get(1);
                                    let current_path = match some_path_name {
                                        Some(path) => path,
                                        None => "",
                                    };
                                    del_content(current_path.to_string(), article_id).await;
                                    rerender.set(true);
                                });
                            }
                        };

                        let change_released = {
                            let history = history.clone();
                            let article_id = article_id.clone();
                            let rerender = rerender.clone();
                            let article_content_string = article_content_string.clone();
                            move |_| {
                                let rerender = rerender.clone();
                                let article_content_string = article_content_string.clone();
                                let article_id = article_id.clone();
                                let path_name = history.location().pathname();
                                spawn_local(async move {
                                    let path_name_vec: Vec<&str> = path_name.split('/').collect();
                                    let some_path_name = path_name_vec.get(1);
                                    let current_path = match some_path_name {
                                        Some(path) => path,
                                        None => "",
                                    };
                                    update_released(
                                        current_path.to_string(),
                                        article_content_string,
                                        article_id,
                                    )
                                    .await;
                                    rerender.set(true);
                                });
                            }
                        };

                        let card = html! {
                            <div class="card" >
                                <img onclick={go_view.clone()} src={article_content.thumbnail}/>
                                <time class="small-text">{article_content.updated_at}</time>
                                <h2 onclick={go_view}>{article_content.title}</h2>
                                if is_signed_result {
                                    <div class="card-buttons">
                                        <button onclick={edit_article}>{"Edit"}</button>
                                        <button onclick={del_article}>{"Delete"}</button>
                                        if article_content.released {
                                            <button onclick={change_released}>{"Public"}</button>
                                        } else {
                                            <button onclick={change_released}>{"Private"}</button>
                                        }
                                    </div>
                                }
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
        let rerender = rerender.clone();
        let card_vnode = card_vnode.clone();
        let current_page = current_page.clone();
        let article_type = article_type.clone();
        let is_signed = is_signed.clone();
        let query_content = query_content.clone();
        move |_| {
            spawn_local(async move {
                let article_contents_value;
                if article_type == "tag" {
                    article_contents_value = fetch_query_contents(
                        article_type.clone(),
                        query_content,
                        props_current_page,
                        limit_num,
                    )
                    .await
                    .as_string();
                } else {
                    article_contents_value = fetch_article_contents(
                        article_type.clone(),
                        props_current_page,
                        limit_num,
                        is_signed,
                    )
                    .await
                    .as_string();
                }

                let article_contents_value = match article_contents_value {
                    Some(article_content) => article_content,
                    None => "".to_string(),
                };
                let article_contents_result: Result<Vec<Article>> =
                    serde_json::from_str(&article_contents_value);

                let article_contents_result = match article_contents_result {
                    Ok(article_contents) => article_contents,
                    Err(_) => vec![],
                };

                let mut vnode: Vec<VNode> = Vec::new();

                for article_content in article_contents_result {
                    let article_id = article_content.id.clone();
                    let article_content_string =
                        serde_json::to_string(&article_content.clone()).unwrap();
                    let go_view = {
                        let history = history.clone();
                        let article_id = article_id.clone();
                        let article_type = article_type.clone();
                        move |_| {
                            let article_type = article_type.clone();
                            let article_id = article_id.clone();
                            if article_type == "blog" || article_type == "tag" {
                                history.push(ViewBlogRoute::View { id: article_id });
                            } else if article_type == "works" {
                                history.push(ViewWorkRoute::View { id: article_id });
                            }
                        }
                    };
                    let edit_article = {
                        let history = history.clone();
                        let article_id = article_id.clone();
                        move |_| {
                            let article_id = article_id.clone();
                            let path_name = history.location().pathname();
                            let path_name_vec: Vec<&str> = path_name.split('/').collect();
                            let some_path_name = path_name_vec.get(1);
                            let current_path = match some_path_name {
                                Some(path) => path,
                                None => "",
                            };
                            if current_path == "blog" {
                                history.push(AdminBlogRoute::AdminArticleEdit { id: article_id });
                            } else if current_path == "works" {
                                history.push(AdminWorkRoute::AdminArticleEdit { id: article_id });
                            }
                        }
                    };
                    let del_article = {
                        let history = history.clone();
                        let article_id = article_id.clone();
                        let rerender = rerender.clone();
                        move |_| {
                            let article_id = article_id.clone();
                            let path_name = history.location().pathname();
                            let rerender = rerender.clone();
                            spawn_local(async move {
                                let path_name_vec: Vec<&str> = path_name.split('/').collect();
                                let some_path_name = path_name_vec.get(1);
                                let current_path = match some_path_name {
                                    Some(path) => path,
                                    None => "",
                                };
                                del_content(current_path.to_string(), article_id).await;
                                rerender.set(true);
                            });
                        }
                    };

                    let change_released = {
                        let history = history.clone();
                        let rerender = rerender.clone();
                        let article_id = article_id.clone();
                        let article_content_string = article_content_string.clone();
                        move |_| {
                            let rerender = rerender.clone();
                            let article_content_string = article_content_string.clone();
                            let article_id = article_id.clone();
                            let path_name = history.location().pathname();
                            spawn_local(async move {
                                let path_name_vec: Vec<&str> = path_name.split('/').collect();
                                let some_path_name = path_name_vec.get(1);
                                let current_path = match some_path_name {
                                    Some(path) => path,
                                    None => "",
                                };
                                update_released(
                                    current_path.to_string(),
                                    article_content_string,
                                    article_id,
                                )
                                .await;
                                rerender.set(true);
                            });
                        }
                    };

                    let card = html! {
                        <div class="card" >
                            <img onclick={go_view.clone()} src={article_content.thumbnail}/>
                            <time class="small-text">{article_content.updated_at}</time>
                            <h2 onclick={go_view}>{article_content.title}</h2>
                            if is_signed {
                                <div class="card-buttons">
                                    <button onclick={edit_article}>{"Edit"}</button>
                                    <button onclick={del_article}>{"Delete"}</button>
                                    if article_content.released {
                                        <button onclick={change_released}>{"Public"}</button>
                                    } else {
                                        <button onclick={change_released}>{"Private"}</button>
                                    }
                                </div>
                            }
                        </div>
                    };
                    vnode.push(card);
                }
                rerender.set(false);
                card_vnode.set(vnode);
                current_page.set(props_current_page);
            });
        }
    };

    if props.current_page != *current_page || *rerender {
        change_article("");
    }
    html! {
        <>
            {card_vnode.to_vec()}
        </>
    }
}
