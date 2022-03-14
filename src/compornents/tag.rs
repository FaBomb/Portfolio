use js_bridge::fetch_tags;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
use yew::virtual_dom::VNode;
use yew::{function_component, html, use_effect_with_deps, use_state};

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
    let init_tags = Tags { tags: vec![] };
    let tags = use_state(|| init_tags);

    {
        let tags = tags.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let tags_value = fetch_tags().await.as_string();
                    match tags_value {
                        Some(tags_val) => {
                            let tags_result: Result<Tags> = serde_json::from_str(&tags_val);
                            let tags_result = match tags_result {
                                Ok(tags) => tags,
                                Err(_) => Tags { tags: vec![] },
                            };
                            tags.set(tags_result);
                        }
                        None => {
                            log::info!("{:?}", "fetch_tags null");
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let mut tag_vnodes: Vec<VNode> = Vec::new();
    let tags = &*tags.clone();
    let tags_vec = &tags.tags;
    for tag in tags_vec {
        let tag_vnode = html! {
            <a href={format!("/blog/1?tag={}",tag.clone())}>{tag.clone()}</a>
        };
        tag_vnodes.push(tag_vnode);
    }

    html! {
        <>
            {tag_vnodes}
        </>
    }
}
