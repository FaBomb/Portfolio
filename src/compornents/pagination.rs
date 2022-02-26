use crate::routing::BlogRoute;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub current_page: u8,
    pub page_size: u8,
}

#[function_component(Pagination)]
pub fn pagination(props: &RenderedAtProps) -> Html {
    let mut pagination: Vec<VNode> = Vec::new();
    for page in 1..props.page_size {
        let page_change = {
            let history = use_history().unwrap();
            move |_| {
                history.push(BlogRoute::Blog {
                    page: page.to_string(),
                });
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
