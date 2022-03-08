use crate::routing::{BlogRoute, WorkRoute};
use yew::virtual_dom::VNode;
use yew::{function_component, html, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub article_type: String,
    pub current_page: u8,
    pub page_size: u8,
}

#[function_component(Pagination)]
pub fn pagination(props: &RenderedAtProps) -> Html {
    let mut pagination: Vec<VNode> = Vec::new();
    let article_type = props.article_type.clone();
    let current_page = props.current_page.clone();

    let back_page = {
        let history = use_history().unwrap();
        let article_type = article_type.clone();
        let current_page = current_page.clone();
        move |_| {
            let next_page = current_page - 1;
            if article_type == "blog" {
                history.push(BlogRoute::Blog {
                    page: next_page.to_string(),
                });
            } else {
                history.push(WorkRoute::Work {
                    page: next_page.to_string(),
                });
            }
        }
    };

    let forward_page = {
        let history = use_history().unwrap();
        let article_type = article_type.clone();
        let current_page = current_page.clone();
        move |_| {
            let next_page = current_page + 1;
            if article_type == "blog" {
                history.push(BlogRoute::Blog {
                    page: next_page.to_string(),
                });
            } else {
                history.push(WorkRoute::Work {
                    page: next_page.to_string(),
                });
            }
        }
    };

    for page in 1..props.page_size {
        let page_change = {
            let history = use_history().unwrap();
            let article_type = article_type.clone();
            move |_| {
                if article_type == "blog" {
                    history.push(BlogRoute::Blog {
                        page: page.to_string(),
                    });
                } else {
                    history.push(WorkRoute::Work {
                        page: page.to_string(),
                    });
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
