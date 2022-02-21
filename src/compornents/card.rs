// use crate::routing::AppRoute;
// use js_bridge::fetch_article_contents;
// use serde::{Deserialize, Serialize};
// use serde_json::Result;
// use wasm_bindgen_futures::spawn_local;
// use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};
// use yew_router::prelude::*;

// #[derive(Properties, PartialEq)]
// pub struct RenderedAtProps {
//     pub id: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Article {
//     category: String,
//     tags: Vec<String>,
//     thumbnail: String,
//     title: String,
//     content: String,
//     released: bool,
//     images: Vec<String>,
//     updated_at: String,
// }

// #[function_component(Card)]
// pub fn card(props: &RenderedAtProps) -> Html {
//     let article_init = Article {
//         category: String::from(""),
//         tags: Vec::new(),
//         thumbnail: String::from(""),
//         title: String::from(""),
//         content: String::from(""),
//         released: false,
//         images: Vec::new(),
//         updated_at: String::from(""),
//     };
//     let article = use_state(|| article_init);
//     let article_id = props.id.clone();
//     let history = use_history().unwrap();
//     let go_view = Callback::once(move |_| {
//         history.push(AppRoute::View {
//             id: article_id.to_string(),
//         })
//     });

//     let article_id = props.id.clone();
//     log::info!("{:?}", article_id);
//     {
//         let article = article.clone();
//         use_effect_with_deps(
//             move |_| {
//                 let article = article.clone();
//                 spawn_local(async move {
//                     let article_content_value =
//                         fetch_article_contents("blog".to_string(), article_id)
//                             .await
//                             .as_string()
//                             .unwrap();
//                     let article_content: Result<Article> =
//                         serde_json::from_str(&article_content_value);
//                     article.set(article_content.unwrap());
//                 });
//                 || ()
//             },
//             (),
//         );
//     }
//     let thumbnail = &article.thumbnail;
//     html! {
//         <div class="card" onclick={go_view}>
//             <img src={thumbnail.to_string()}/>
//             <time>{&article.updated_at}</time>
//             <h1>{&article.title}</h1>
//         </div>
//     }
// }
