use crate::admin::admin_article_edit::markdown;
use crate::compornents::{footer::Footer, header::Header};
use crate::routing::{AppRoute, BlogRoute};
use js_bridge::fetch_article_content_from_id;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
use yew::virtual_dom::VNode;
use yew::{function_component, html, use_effect_with_deps, use_state, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    category: String,
    tags: Vec<String>,
    thumbnail: String,
    title: String,
    content: String,
    released: bool,
    images: Vec<String>,
    updated_at: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    tag: String,
}

#[function_component(View)]
pub fn view(props: &RenderedAtProps) -> Html {
    let article_init = Article {
        category: String::from(""),
        tags: Vec::new(),
        thumbnail: String::from(""),
        title: String::from(""),
        content: String::from(""),
        released: false,
        images: Vec::new(),
        updated_at: String::from(""),
    };
    let article = use_state(|| article_init);
    let history = use_history().unwrap();
    let id = props.id.clone();

    let path_name = history.location().pathname();
    let path_name_vec: Vec<&str> = path_name.split('/').collect();
    let some_path_name = path_name_vec.get(1);
    let current_path = match some_path_name {
        Some(path) => path,
        None => "",
    };
    let article_type;
    if current_path == "view_blog" {
        article_type = "blog".to_string();
    } else {
        article_type = "works".to_string();
    }

    {
        let article = article.clone();
        use_effect_with_deps(
            move |_| {
                let article = article.clone();
                spawn_local(async move {
                    let article_content_value = fetch_article_content_from_id(article_type, id)
                        .await
                        .as_string();
                    match article_content_value {
                        Some(article_content) => {
                            let article_result: Result<Article> =
                                serde_json::from_str(&article_content);
                            article.set(article_result.unwrap());
                        }
                        None => {
                            history.push(AppRoute::NotFound);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let mut tags: Vec<VNode> = Vec::new();
    for tag in &article.tags {
        let go_tag = {
            let history = use_history().unwrap();
            let tag = tag.clone();
            move |_| {
                let history = history.clone();
                let tag_struct = Tag { tag: tag.clone() };
                history
                    .push_with_query(
                        BlogRoute::Blog {
                            page: "1".to_string(),
                        },
                        tag_struct,
                    )
                    .unwrap()
            }
        };
        let tag = html! {
            <li onclick={go_tag}>{tag}</li>
        };
        tags.push(tag);
    }

    let markdown_vnode = markdown(&article.content);
    html! {
        <>
            <Header/>
            <div class="view">
                <div class="title-box">
                    <p class="small-text">
                        <i class="fa-solid fa-clock"></i>
                        {&article.updated_at}<br/>
                        {"- "}{&article.category}{" -"}
                    </p>
                    <h1>{&article.title}</h1>
                    <img src={article.thumbnail.to_string()} alt="thumbnail" class="thumbnail" />
                    <ul>{tags}</ul>
                </div>
                <div class="content-box">
                    {markdown_vnode}
                </div>
            </div>
            <Footer/>
        </>
    }
}
