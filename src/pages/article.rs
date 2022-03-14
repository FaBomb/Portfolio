use crate::compornents::{
    card::Card, footer::Footer, header::Header, pagination::Pagination, tag::TagCompornent,
};
use js_bridge::{fetch_article_size, fetch_query_size, is_signed_in};
use serde::{Deserialize, Serialize};

use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub page: String,
    pub article_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Tags {
    tags: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    tag: String,
}

#[function_component(Article)]
pub fn article(props: &RenderedAtProps) -> Html {
    let page_clone = props.page.clone();
    let page_u8: u8 = page_clone.parse::<u8>().unwrap();
    let page_size = use_state(|| 1);
    let mut article_type = props.article_type.clone();
    let is_signed = use_state(|| false);

    let limit_num = 6;
    let history = use_history().unwrap();

    let mut query: String = history.location().search();
    let mut query_name: String = "".to_string();
    let mut query_content: String = "".to_string();
    if query.len() > 1 {
        query.remove(0);
        let query_vec: Vec<&str> = query.split('=').collect();
        let some_query_name = query_vec.get(0);
        query_name = match some_query_name {
            Some(query) => query.to_string(),
            None => "".to_string(),
        };
        let some_query_content_name = query_vec.get(1);
        query_content = match some_query_content_name {
            Some(content) => content.to_string(),
            None => "".to_string(),
        };
        article_type = query_name.clone();
    }

    {
        let page_size = page_size.clone();
        let article_type = article_type.clone();
        let is_signed = is_signed.clone();
        let query_content = query_content.clone();
        let query_name = query_name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let prop_article_type = &*article_type;
                    let result = is_signed_in("_").await.as_bool().unwrap();
                    is_signed.set(result);

                    let fetch_page_size: u8;
                    if query_name.len() == 0 {
                        fetch_page_size = fetch_article_size(prop_article_type.to_string(), result)
                            .await
                            .as_f64()
                            .unwrap()
                            .round() as u8;
                    } else if query_content.len() != 0 {
                        fetch_page_size =
                            fetch_query_size(query_name.to_string(), query_content.to_string())
                                .await
                                .as_f64()
                                .unwrap()
                                .round() as u8;
                    } else {
                        fetch_page_size = 1;
                    }
                    let new_page_size;
                    if fetch_page_size % limit_num != 0 {
                        new_page_size = fetch_page_size / limit_num + 2;
                    } else {
                        new_page_size = fetch_page_size / limit_num + 1;
                    }
                    page_size.set(new_page_size);
                });
                || ()
            },
            (),
        );
    }
    let article_type = &*article_type.clone();
    html! {
        <>
            <Header/>
            <div class="article">
                if article_type == "blog" {
                    <h1>{"- "}{article_type}{" -"}</h1>
                    <p class="small-text">{"ブログ記事"}</p>
                } else if article_type == "works" {
                    <h1>{"- "}{article_type}{" -"}</h1>
                    <p class="small-text">{"過去に制作した作品"}</p>
                } else if article_type == "tag" {
                    <h1>{"- "}{query_content.clone()}{" -"}</h1>
                    <p class="small-text">{"タグ「"}{query_content.clone()}{"」が付いているブログ記事"}</p>
                }
                <div class="cards">
                    <Card current_page={page_u8} limit_num={limit_num}
                    article_type={article_type.to_string()} is_signed={*is_signed}
                    query_content={query_content.clone()}/>
                </div>
                <ul class="pagination">
                    <Pagination article_type={article_type.to_string()}
                    current_page={page_u8} page_size={*page_size} query_content={query_content.clone()}/>
                </ul>
                if article_type == "blog" || article_type == "tag" {
                    <div class="tag-box">
                        <h1>{"- "}{"Tags"}{" -"}</h1>
                        <p class="small-text">{"タグ一覧"}</p>
                        <div class="tags">
                            <TagCompornent/>
                        </div>
                    </div>
                }
            </div>
            <Footer/>
        </>
    }
}
