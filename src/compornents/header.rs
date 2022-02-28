use crate::routing::{AdminBlogRoute, AdminRoute, AdminWorkRoute, AppRoute, BlogRoute, WorkRoute};
use js_bridge::{is_signed_in, sign_out};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback};
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let is_signed = use_state(|| false);

    let history = use_history().unwrap();
    let go_home = Callback::from(move |_| history.push(AppRoute::Home));
    let history = use_history().unwrap();
    let go_blog = Callback::from(move |_| {
        history.push(BlogRoute::Blog {
            page: "1".to_string(),
        })
    });
    let history = use_history().unwrap();
    let go_work = Callback::from(move |_| {
        history.push(WorkRoute::Work {
            page: "1".to_string(),
        })
    });
    let history = use_history().unwrap();
    let go_new_blog = Callback::from(move |_| {
        history.push(AdminBlogRoute::AdminArticleEdit {
            id: "new".to_string(),
        })
    });
    let history = use_history().unwrap();
    let go_new_work = Callback::from(move |_| {
        history.push(AdminWorkRoute::AdminArticleEdit {
            id: "new".to_string(),
        })
    });

    let history = use_history().unwrap();

    let path_name = history.location().pathname();
    let path_name_vec: Vec<&str> = path_name.split('/').collect();
    let some_path_name = path_name_vec.get(1);
    // let some_admin_path_name = path_name_vec.get(2);
    let current_path_name = match some_path_name {
        Some(path) => path,
        None => "",
    };
    // let current_admin_path_name = match some_admin_path_name {
    //     Some(path) => path,
    //     None => "",
    // };

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
                    history.push(AdminRoute::Admin);
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
                if current_path_name == "" {
                    <button onclick={go_blog}>{ "Blog" }</button>
                    <button onclick={go_work}>{ "Work" }</button>
                } else if current_path_name == "blog" {
                    <button onclick={go_home}>{ "Home" }</button>
                    <button onclick={go_work}>{ "Work" }</button>
                    if *is_signed {
                        <button onclick={go_new_blog}>{ "NewBlog" }</button>
                        <button onclick={go_new_work}>{ "NewWork" }</button>
                        <button onclick={onclick_sign_out}>{ "SignOut" }</button>
                    }
                } else if current_path_name == "work" {
                    <button onclick={go_home}>{ "Home" }</button>
                    <button onclick={go_blog}>{ "Blog" }</button>
                    if *is_signed {
                        <button onclick={go_new_blog}>{ "NewBlog" }</button>
                        <button onclick={go_new_work}>{ "NewWork" }</button>
                        <button onclick={onclick_sign_out}>{ "SignOut" }</button>
                    }
                } else if current_path_name == "admin" {
                    <button onclick={go_home}>{ "Home" }</button>
                    <button onclick={go_blog}>{ "Blog" }</button>
                    <button onclick={go_work}>{ "Work" }</button>
                    if *is_signed {
                        <button onclick={go_new_blog}>{ "NewBlog" }</button>
                        <button onclick={go_new_work}>{ "NewWork" }</button>
                        <button onclick={onclick_sign_out}>{ "SignOut" }</button>
                    }
                } else {
                    <button onclick={go_home}>{ "Home" }</button>
                    <button onclick={go_blog}>{ "Blog" }</button>
                    <button onclick={go_work}>{ "Work" }</button>
                }
            </nav>
        </header>
    }
}
