use crate::routing::{AdminBlogRoute, AppRoute};
use js_bridge::{is_signed_in, sign_out};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback};
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let is_signed = use_state(|| false);

    let history = use_history().unwrap();
    let go_blog = Callback::from(move |_| history.push(AppRoute::Blog));
    let history = use_history().unwrap();
    let go_home = Callback::from(move |_| history.push(AppRoute::Home));
    let history = use_history().unwrap();
    let go_works = Callback::from(move |_| history.push(AppRoute::Works));
    let history = use_history().unwrap();
    let go_admin_blog = Callback::from(move |_| history.push(AdminBlogRoute::AdminBlog));
    let history = use_history().unwrap();
    let go_admin_work = Callback::from(move |_| history.push(AppRoute::AdminWork));
    let history = use_history().unwrap();
    let go_article_edit = Callback::from(move |_| history.push(AppRoute::AdminArticleEdit));

    let history = use_history().unwrap();
    let current_path_name = history.location().pathname();

    let onclick_sign_out = {
        let is_signed = is_signed.clone();
        let history = history.clone();
        move |_| {
            let history = history.clone();
            let is_signed = is_signed.clone();
            spawn_local(async move {
                let result = sign_out("_").await;
                if result.as_bool().unwrap() {
                    is_signed.set(!*is_signed);
                    log::info!("sign out");
                    history.push(AppRoute::Admin);
                }
            });
        }
    };

    {
        let is_signed = is_signed.clone();
        use_effect_with_deps(
            move |_| {
                let is_signed = is_signed.clone();
                spawn_local(async move {
                    let result = is_signed_in("_").await;
                    is_signed.set(result.as_bool().unwrap());
                });
                || ()
            },
            (),
        );
    }

    html! {
        <header>
            <h2>{ "Header" }</h2>
            <nav>
                if current_path_name == "/admin_blog" {
                    <button onclick={onclick_sign_out}>{ "SignOut" }</button>
                    <button onclick={go_article_edit}>{ "ArticleEdit" }</button>
                    <button onclick={go_admin_work}>{ "AdminWork" }</button>
                } else if current_path_name == "/admin_article_edit" {
                    <button onclick={onclick_sign_out}>{ "SignOut" }</button>
                    <button onclick={go_admin_blog}>{ "AdminBlog" }</button>
                    <button onclick={go_admin_work}>{ "AdminWork" }</button>
                } else if current_path_name == "/admin_work" {
                    <button onclick={onclick_sign_out}>{ "SignOut" }</button>
                    <button onclick={go_admin_blog}>{ "AdminBlog" }</button>
                } else if current_path_name == "/" {
                    <button onclick={go_blog}>{ "Blog" }</button>
                    <button onclick={go_works}>{ "Works" }</button>
                } else if current_path_name == "/blog" {
                    <button onclick={go_home}>{ "Home" }</button>
                    <button onclick={go_works}>{ "Works" }</button>
                } else if current_path_name == "/works" {
                    <button onclick={go_home}>{ "Home" }</button>
                    <button onclick={go_blog}>{ "Blog" }</button>
                } else if *is_signed {
                    <button onclick={go_article_edit}>{ "ArticleEdit" }</button>
                    <button onclick={go_admin_blog}>{ "AdminBlog" }</button>
                    <button onclick={go_admin_work}>{ "AdminWork" }</button>
                } else {
                    <button onclick={go_home}>{ "Home" }</button>
                    <button onclick={go_blog}>{ "Blog" }</button>
                    <button onclick={go_works}>{ "Works" }</button>
                }
            </nav>
        </header>
    }
}
