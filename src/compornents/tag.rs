use crate::routing::BlogRoute;
use js_bridge::fetch_tags;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
use yew::virtual_dom::VNode;
use yew::{function_component, html, use_effect_with_deps, use_state};
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tags {
    tags: Vec<String>,
}

#[function_component(TagCompornent)]
pub fn tag() -> Html {
    let init_tag_vnode: Vec<VNode> = Vec::new();
    let tag_vnode = use_state(|| init_tag_vnode);
    let history = use_history().unwrap();

    {
        let tag_vnode = tag_vnode.clone();
        let history = history.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let tags_value = fetch_tags().await.as_string();
                    let mut tag_vnode_box: Vec<VNode> = Vec::new();
                    match tags_value {
                        Some(tags_val) => {
                            let tags_result: Result<Tags> = serde_json::from_str(&tags_val);
                            match tags_result {
                                Ok(tags) => {
                                    for tag in tags.tags {
                                        let go_tag = {
                                            let tag = tag.clone();
                                            let history = history.clone();
                                            move |_| {
                                                let query = Tag {
                                                    tag: tag.to_string(),
                                                };
                                                history.push(BlogRoute::Blog {
                                                    page: "2".to_string(),
                                                });
                                                history
                                                    .push_with_query(
                                                        BlogRoute::Blog {
                                                            page: "1".to_string(),
                                                        },
                                                        query,
                                                    )
                                                    .unwrap();
                                            }
                                        };
                                        let vnode = html! {
                                            <a onclick={go_tag}>{tag.clone()}</a>
                                        };
                                        tag_vnode_box.push(vnode);
                                    }
                                }
                                Err(_) => {
                                    log::info!("{:?}", "fetch_tags Err");
                                }
                            };
                        }
                        None => {
                            log::info!("{:?}", "fetch_tags null");
                        }
                    };

                    tag_vnode.set(tag_vnode_box);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            {tag_vnode.to_vec()}
        </>
    }
}
