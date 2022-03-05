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
                <li style="color:red">{page}</li>
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
            {pagination.to_vec()}
        </>
    }
}
