use crate::compornents::{footer::Footer, header::Header};
use crate::routing::{AdminRoute, BlogRoute, WorkRoute};
use js_bridge::{
    del_category, del_tag, fetch_all_article_content_from_id, fetch_categories, fetch_tags,
    is_signed_in, set_category, set_content, set_tag, update_content, upload,
};
use pulldown_cmark::{html as markdown_html, Options, Parser};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement, Node};
use yew::virtual_dom::VNode;
use yew::{function_component, functional::*, html, use_effect_with_deps, use_state, Properties};
use yew_router::prelude::*;

pub fn markdown(source_text: &str) -> VNode {
    let markdown_input = source_text;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    let parser = Parser::new_ext(markdown_input, options);
    let mut html_output = String::new();
    markdown_html::push_html(&mut html_output, parser);

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let div = document.create_element("div").unwrap();
    div.set_inner_html(&html_output);
    let node = Node::from(div);
    let vnode = VNode::VRef(node);
    vnode
}

fn pulldown_options(values: Vec<String>, selected: Vec<String>) -> Vec<VNode> {
    let mut options = Vec::new();
    for val in values {
        let val = val.clone();
        if selected.iter().any(|select| select == &val) {
            let pulldown_option = html! {
                <option value={val.clone()} selected=true>{val}</option>
            };
            options.push(pulldown_option);
        } else {
            let pulldown_option = html! {
                <option value={val.clone()}>{val}</option>
            };
            options.push(pulldown_option);
        }
    }
    options
}

#[derive(Serialize, Deserialize, Debug)]
struct NewArticle {
    category: String,
    tags: Vec<String>,
    thumbnail: String,
    title: String,
    content: String,
    released: bool,
    images: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    category: String,
    tags: Vec<String>,
    thumbnail: String,
    title: String,
    content: String,
}

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tags {
    tags: Vec<String>,
}

#[function_component(AdminArticleEdit)]
pub fn admin_article_edit(props: &RenderedAtProps) -> Html {
    let id = props.id.clone();

    let history = use_history().unwrap();
    let path_name = history.location().pathname();
    let path_name_vec: Vec<&str> = path_name.split('/').collect();
    let some_path_name = path_name_vec.get(1);
    let current_path = match some_path_name {
        Some(path) => path,
        None => "",
    };
    let mut article_type = id.clone();
    if current_path == "admin_work" {
        article_type = "works".to_string();
    } else if current_path == "admin_blog" {
        article_type = "blog".to_string();
    }

    let is_signed = use_state(|| true);

    let title = use_state(|| "".to_string());
    let text = use_state(|| "".to_string());
    let thumbnail = use_state(|| {
        "https://firebasestorage.googleapis.com/v0/b/portfolio-7d273.appspot.com/o/no-img.png?alt=media&token=a49234a7-66c8-4984-92fb-c9495be490ab"
        .to_string()
    });

    let init_category = use_state(|| vec!["".to_string()]);
    let init_tags_val: Vec<String> = Vec::new();
    let init_tags = use_state(|| init_tags_val);
    let categories = use_state(Vec::new);
    let tags = use_state(Vec::new);

    let new_category_ref = use_node_ref();
    let select_category_ref = use_node_ref();
    let new_tag_ref = use_node_ref();
    let select_tag_ref = use_node_ref();
    let title_ref = use_node_ref();
    let text_ref = use_node_ref();
    let file_ref = use_node_ref();
    let thumbnail_ref = use_node_ref();

    let markdown_vnode = markdown(&text);
    let pulldown_category_option_vnode =
        pulldown_options(categories.to_vec(), init_category.to_vec());
    let pulldown_tag_option_vnode = pulldown_options(tags.to_vec(), init_tags.to_vec());

    {
        let id = id.clone();
        let title_state = title.clone();
        let init_category = init_category.clone();
        let init_tags = init_tags.clone();
        let thumbnail_state = thumbnail.clone();
        let text = text.clone();
        let history = history.clone();
        let article_type = article_type.clone();
        let is_signed = is_signed.clone();
        let categories = categories.clone();
        let tags = tags.clone();
        use_effect_with_deps(
            move |_| {
                let is_signed = is_signed.clone();
                let categories = categories.clone();
                let tags = tags.clone();
                spawn_local(async move {
                    let result = is_signed_in("_").await.as_bool().unwrap();
                    is_signed.set(result);
                    if !result {
                        history.push(AdminRoute::Admin);
                    }
                    let article_content_value = fetch_all_article_content_from_id(article_type, id)
                        .await
                        .as_string();
                    match article_content_value {
                        Some(article_content) => {
                            let article_result: Result<Article> =
                                serde_json::from_str(&article_content);
                            let article_result = article_result.unwrap();
                            let category = article_result.category.clone();
                            let tags = article_result.tags.clone();
                            let thumbnail = article_result.thumbnail.clone();
                            let title = article_result.title.clone();
                            let content = article_result.content.clone();
                            init_category.set(vec![category]);
                            init_tags.set(tags);
                            thumbnail_state.set(thumbnail);
                            title_state.set(title);
                            text.set(content);
                        }
                        None => {
                            log::info!("{:?}", "fetch_all_article_content_from_id null");
                        }
                    }

                    let categories_value = fetch_categories().await.as_string();
                    let categories_value = match categories_value {
                        Some(categories) => categories,
                        None => "".to_string(),
                    };
                    let categories_vec: Result<Vec<String>> =
                        serde_json::from_str(&categories_value);
                    let categories_vec = match categories_vec {
                        Ok(categories) => categories,
                        Err(_) => vec![],
                    };
                    categories.set(categories_vec);

                    let tags_value = fetch_tags().await.as_string();
                    let tags_value = match tags_value {
                        Some(tags) => tags,
                        None => "".to_string(),
                    };
                    let tags_result: Result<Tags> = serde_json::from_str(&tags_value);
                    let tags_result = match tags_result {
                        Ok(tags) => tags,
                        Err(_) => Tags { tags: vec![] },
                    };
                    tags.set(tags_result.tags);
                });
                || ()
            },
            (),
        );
    }

    let oninput_title = {
        let title_ref = title_ref.clone();
        let title = title.clone();
        move |_| {
            if let Some(title_value) = title_ref.cast::<HtmlInputElement>() {
                let title_value = title_value.value();
                title.set(title_value);
            }
        }
    };

    let oninput_value = {
        let text_ref = text_ref.clone();
        let text = text.clone();
        move |_| {
            if let Some(text_value) = text_ref.cast::<HtmlInputElement>() {
                let text_value = text_value.value();
                text.set(text_value);
            }
        }
    };

    let onchange_file = {
        let text_ref = text_ref.clone();
        let file_ref = file_ref.clone();
        let text = text.clone();
        move |_| {
            let text = text.clone();
            if let Some(file) = file_ref.cast::<HtmlInputElement>() {
                if let Some(text_element) = text_ref.cast::<HtmlInputElement>() {
                    spawn_local(async move {
                        let files = file.files();
                        let mut file_name: Option<web_sys::File> = None;
                        let result = match files {
                            Some(files) => match files.item(0) {
                                Some(file) => {
                                    file_name = Some(file.clone());
                                    let result = upload(file).await;
                                    result.as_string().unwrap()
                                }
                                None => "".to_string(),
                            },
                            None => "".to_string(),
                        };
                        let file_type = match file_name {
                            Some(file) => file.type_().to_string(),
                            None => "".to_string(),
                        };
                        let insert_index: u32 = match text_element.selection_start() {
                            Ok(index_option) => match index_option {
                                Some(index) => index,
                                None => 0,
                            },
                            Err(_) => 0,
                        };
                        let text_value = text_element.value();
                        let mut before_text = "".to_string();
                        let mut after_text = "".to_string();
                        for (i, text) in text_value.chars().enumerate() {
                            if i < insert_index as usize {
                                before_text.push(text);
                            } else {
                                after_text.push(text);
                            }
                        }
                        if file_type == "video/mp4" {
                            let video_url =
                                ["\n<video src='", "' controls></video>\n"].join(&result);
                            let new_text = [before_text, after_text].join(&video_url);
                            text_element.set_value(&new_text);
                            text.set(new_text);
                        } else if file_type != "" {
                            let image_url = ["\n![image_name](", ")\n"].join(&result);
                            let new_text = [before_text, after_text].join(&image_url);
                            text_element.set_value(&new_text);
                            text.set(new_text);
                        }
                    });
                }
            }
        }
    };

    let onchange_thumbnail = {
        let thumbnail_ref = thumbnail_ref.clone();
        let thumbnail = thumbnail.clone();
        move |_| {
            let thumbnail = thumbnail.clone();
            if let Some(file) = thumbnail_ref.cast::<HtmlInputElement>() {
                spawn_local(async move {
                    let file = file.files().unwrap().item(0).unwrap();
                    let result = upload(file).await;
                    let result = result.as_string().unwrap();

                    thumbnail.set(result);
                });
            }
        }
    };

    let post = {
        let select_category_ref = select_category_ref.clone();
        let id = id.clone();
        let article_type = article_type.clone();
        let select_tag_ref = select_tag_ref.clone();
        let title = title.clone();
        let thumbnail = thumbnail.clone();
        let text = text.clone();
        let thumbnail = thumbnail.clone();
        let history = history.clone();
        move |_| {
            let history = history.clone();
            let article_type = article_type.clone();
            let id = id.clone();
            let title = title.clone();
            let thumbnail = thumbnail.clone();
            let text = text.clone();
            let thumbnail = thumbnail.clone();
            if let Some(select_category) = select_category_ref.cast::<HtmlInputElement>() {
                if let Some(select_tags) = select_tag_ref.cast::<HtmlSelectElement>() {
                    spawn_local(async move {
                        let re_all =
                            Regex::new(r"!\[.*]\(https?://[\w/:%#\$&\?\(\)~\.=\+\-]+\)").unwrap();
                        let re_part = Regex::new(r"https?://[\w/:%#\$&\?\(\)~\.=\+\-]+").unwrap();
                        let mut save_urls = Vec::new();
                        for caps in re_all.captures_iter(&*text) {
                            let image_url = re_part
                                .captures(caps.get(0).unwrap().as_str())
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .as_str();
                            let image_url = image_url.split_at(image_url.len() - 1).0;
                            save_urls.push(image_url.to_string());
                        }
                        let re_video_all =
                            Regex::new(r"src='https?://[\w/:%#\$&\?\(\)~\.=\+\-]+").unwrap();
                        for video_cap in re_video_all.captures_iter(&*text) {
                            let video_url = video_cap.get(0).unwrap().as_str();
                            let video_url: Vec<&str> = video_url.split("src='").collect();
                            match video_url.get(1) {
                                Some(url) => {
                                    save_urls.push(url.to_string());
                                }
                                None => {}
                            }
                        }
                        let thumbnail_string = &*thumbnail;
                        save_urls.push(thumbnail_string.to_string());

                        let text_string = &*text;

                        let select_tags_collection = select_tags.selected_options();
                        let mut tags_vec = Vec::new();
                        for i in 0..select_tags_collection.length() {
                            if let Some(select_tag_item) = select_tags_collection.item(i) {
                                if let Some(select_tag_text) = select_tag_item.text_content() {
                                    tags_vec.push(select_tag_text);
                                }
                            };
                        }

                        let ariticle = NewArticle {
                            category: select_category.value(),
                            tags: tags_vec.to_vec(),
                            thumbnail: thumbnail.to_string(),
                            title: title.to_string(),
                            content: text_string.to_string(),
                            released: false,
                            images: save_urls,
                        };
                        let serialized_article = serde_json::to_string(&ariticle).unwrap();

                        if id == "new" {
                            set_content(article_type.clone(), serialized_article).await;
                            if article_type == "works" {
                                history.push(WorkRoute::Work {
                                    page: "1".to_string(),
                                })
                            } else if article_type == "blog" {
                                history.push(BlogRoute::Blog {
                                    page: "1".to_string(),
                                })
                            }
                        } else {
                            update_content(article_type, serialized_article, id).await;
                        }
                    });
                }
            }
        }
    };

    let add_category = {
        let new_category_ref = new_category_ref.clone();
        let categories = categories.clone();
        move |_| {
            let categories = categories.clone();
            if let Some(new_category) = new_category_ref.cast::<HtmlInputElement>() {
                spawn_local(async move {
                    let new_category = new_category.value();
                    set_category(new_category.clone()).await;
                    let mut temp_category = (*categories).clone();
                    temp_category.push(new_category);
                    categories.set(temp_category);
                });
            }
        }
    };
    let del_category = {
        let select_category_ref = select_category_ref.clone();
        move |_| {
            if let Some(select_category) = select_category_ref.cast::<HtmlInputElement>() {
                spawn_local(async move {
                    let del_category_value = select_category.value();
                    del_category(del_category_value).await;
                });
            }
        }
    };
    let add_tag = {
        let new_tag_ref = new_tag_ref.clone();
        let tags = tags.clone();
        move |_| {
            let tags = tags.clone();
            if let Some(new_tag) = new_tag_ref.cast::<HtmlInputElement>() {
                spawn_local(async move {
                    let new_tag = new_tag.value();
                    set_tag(new_tag.clone()).await;
                    let mut temp_tag = (*tags).clone();
                    temp_tag.push(new_tag);
                    tags.set(temp_tag);
                });
            }
        }
    };
    let del_tag = {
        let select_tag_ref = select_tag_ref.clone();
        move |_| {
            if let Some(select_tag) = select_tag_ref.cast::<HtmlInputElement>() {
                spawn_local(async move {
                    let del_tag_value = select_tag.value();
                    del_tag(del_tag_value).await;
                });
            }
        }
    };

    let thumbnail_string = &*thumbnail;
    let title_string = &*title;

    html! {
        <>
            <Header/>
            <div class="article-edit">
                <h1>{"- "} { article_type.clone() } {" Edit -" }</h1>
                <div class="support-tools">
                    <div class="image-upload">
                        <label>
                            <input ref={thumbnail_ref} type="file" accept="image/png, image/jpeg"
                             onchange={onchange_thumbnail} /> {"Thumbnail"}
                        </label>
                    </div>
                    <div class="select-box">
                        <h3>{"Category"}</h3>
                        <label for="category-select">
                            <div class="add-select">
                                <input type="text" ref={new_category_ref}/>
                                <button onclick={add_category}>{"+"}</button>
                            </div>
                            <div class="choose-select">
                                <div class="selecter">
                                    <select name="category" ref={select_category_ref} id="category-select">
                                        {pulldown_category_option_vnode}
                                    </select>
                                </div>
                                <button onclick={del_category}>{"-"}</button>
                            </div>
                        </label>

                        if article_type == "works" {
                            <h3 class="works-edit">{"Tags"}</h3>
                            <label for="tag-select" class="works-edit">
                                <div class="add-select">
                                    <input type="text" ref={new_tag_ref}/>
                                    <button onclick={add_tag}>{"+"}</button>
                                </div>
                                <div class="choose-select">
                                    <div class="selecter">
                                        <select name="tag" ref={select_tag_ref} size="3" multiple={true} id="tag-select">
                                            {pulldown_tag_option_vnode}
                                        </select>
                                    </div>
                                    <button onclick={del_tag}>{"-"}</button>
                                </div>
                            </label>
                        } else {
                            <h3>{"Tags"}</h3>
                            <label for="tag-select">
                                <div class="add-select">
                                    <input type="text" ref={new_tag_ref}/>
                                    <button onclick={add_tag}>{"+"}</button>
                                </div>
                                <div class="choose-select">
                                    <div class="selecter">
                                        <select name="tag" ref={select_tag_ref} size="3" multiple={true} id="tag-select">
                                            {pulldown_tag_option_vnode}
                                        </select>
                                    </div>
                                    <button onclick={del_tag}>{"-"}</button>
                                </div>
                            </label>
                        }
                        <h3>{"Title"}</h3>
                        <textarea class="title-area" ref={title_ref} oninput={oninput_title} value={title.to_string()} />
                    </div>
                </div>
                <div class="markdown">
                    <div class="editor">
                        <label>
                            <input ref={file_ref} type="file" accept="image/png, image/jpeg, video/mp4"
                            onchange={onchange_file} />{"Up Image"}
                        </label>
                        <textarea ref={text_ref} oninput={oninput_value} value={text.to_string()} />
                    </div>
                    <div class="view">
                        <div class="title-box">
                            <p class="small-text">
                                <i class="fa-solid fa-clock"></i>
                                {"20??????.??????.??????"}
                            </p>
                            <h1>{title_string}</h1>
                            <img src={thumbnail_string.to_string()} alt="thumbnail" class="thumbnail" />
                        </div>
                        <div class="content-box">
                            {markdown_vnode}
                        </div>
                    </div>
                </div>
                if id == "new" {
                    <button onclick={post} class="post-btn">{"Post"}</button>
                } else {
                    <button onclick={post} class="post-btn">{"Edit"}</button>
                }
            </div>
            <Footer/>
        </>
    }
}
