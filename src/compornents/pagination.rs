use crate::routing::{BlogRoute, WorkRoute};
use serde::{Deserialize, Serialize};
use yew::virtual_dom::VNode;
use yew::{function_component, html, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub article_type: String,
    pub current_page: u8,
    pub page_size: u8,
    pub query_content: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    tag: String,
}

#[function_component(Pagination)]
pub fn pagination(props: &RenderedAtProps) -> Html {
    let mut pagination: Vec<VNode> = Vec::new();
    let article_type = props.article_type.clone();
    let current_page = props.current_page.clone();
    let query_content = props.query_content.clone();

    let back_page = {
        let history = use_history().unwrap();
        let article_type = article_type.clone();
        let current_page = current_page.clone();
        let query_content = query_content.clone();
        move |_| {
            let query_content = query_content.clone();
            let next_page = current_page - 1;
            if article_type == "blog" {
                history.push(BlogRoute::Blog {
                    page: next_page.to_string(),
                });
            } else if article_type == "works" {
                history.push(WorkRoute::Work {
                    page: next_page.to_string(),
                });
            } else if article_type == "tag" {
                let tag_struct = Tag {
                    tag: query_content.clone(),
                };
                history
                    .push_with_query(
                        BlogRoute::Blog {
                            page: next_page.to_string(),
                        },
                        tag_struct,
                    )
                    .unwrap();
            }
        }
    };

    let forward_page = {
        let history = use_history().unwrap();
        let article_type = article_type.clone();
        let current_page = current_page.clone();
        let query_content = query_content.clone();
        move |_| {
            let query_content = query_content.clone();
            let next_page = current_page + 1;
            if article_type == "blog" {
                history.push(BlogRoute::Blog {
                    page: next_page.to_string(),
                });
            } else if article_type == "works" {
                history.push(WorkRoute::Work {
                    page: next_page.to_string(),
                });
            } else if article_type == "tag" {
                let tag_struct = Tag { tag: query_content };
                history
                    .push_with_query(
                        BlogRoute::Blog {
                            page: next_page.to_string(),
                        },
                        tag_struct,
                    )
                    .unwrap();
            }
        }
    };

    for page in 1..props.page_size {
        let page_change = {
            let history = use_history().unwrap();
            let article_type = article_type.clone();
            let query_content = query_content.clone();
            move |_| {
                let query_content = query_content.clone();
                if article_type == "blog" {
                    history.push(BlogRoute::Blog {
                        page: page.to_string(),
                    });
                } else if article_type == "works" {
                    history.push(WorkRoute::Work {
                        page: page.to_string(),
                    });
                } else if article_type == "tag" {
                    let tag_struct = Tag { tag: query_content };
                    history
                        .push_with_query(
                            BlogRoute::Blog {
                                page: page.to_string(),
                            },
                            tag_struct,
                        )
                        .unwrap();
                }
            }
        };
        let li_tag;
        if page == props.current_page {
            li_tag = html! {
                <li class="current_page">{page}</li>
            };
        } else {
            li_tag = html! {
                <li onclick={page_change}>{page}</li>
            };
        }
        pagination.push(li_tag);
    }

    html! {
        <>
            if props.current_page != 1 {
                <li onclick={back_page}><div class="allow">{"<"}</div></li>
            }
            {pagination.to_vec()}
            if props.current_page != props.page_size - 1 {
                <li onclick={forward_page}><div class="allow">{">"}</div></li>
            }
        </>
    }
}
