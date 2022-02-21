use crate::admin::admin_article_edit::markdown;
use crate::compornents::{footer::Footer, header::Header};
use crate::routing::AppRoute;
use js_bridge::fetch_article_content_from_id;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
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
    {
        let article = article.clone();
        use_effect_with_deps(
            move |_| {
                let article = article.clone();
                spawn_local(async move {
                    let article_content_value =
                        fetch_article_content_from_id("blog".to_string(), id)
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

    let markdown_vnode = markdown(&article.content);
    html! {
        <>
            <Header/>
            <h1>{ "View" }</h1>
            <div class="markdown view">
                <img src={article.thumbnail.to_string()} alt="thumbnail" />
                <h1>{&article.title}</h1>
                {markdown_vnode}
            </div>
            <Footer/>
        </>
    }
}
