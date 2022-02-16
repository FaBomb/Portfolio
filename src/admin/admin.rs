use crate::compornents::{footer::Footer, header::Header};
use crate::routing::AppRoute;
use js_bridge::{is_signed_in, sign_in};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{function_component, functional::*, html, use_effect_with_deps, use_state};
use yew_router::prelude::*;

#[function_component(Admin)]
pub fn admin() -> Html {
    let history = use_history().unwrap();
    let is_signed = use_state(|| true);

    let mail_address_ref = use_node_ref();
    let password_ref = use_node_ref();

    let onclick_sign_in = {
        let mail_address_ref = mail_address_ref.clone();
        let password_ref = password_ref.clone();
        let history = history.clone();
        let is_signed = is_signed.clone();
        move |_| {
            let history = history.clone();
            let is_signed = is_signed.clone();
            if let Some(mail_address) = mail_address_ref.cast::<HtmlInputElement>() {
                if let Some(password) = password_ref.cast::<HtmlInputElement>() {
                    spawn_local(async move {
                        let result = sign_in(&mail_address.value(), &password.value()).await;
                        if result.as_bool().unwrap() {
                            is_signed.set(!*is_signed);
                            log::info!("sign in");
                            history.push(AppRoute::AdminBlog);
                        }
                    });
                }
            }
        }
    };

    {
        let is_signed = is_signed.clone();
        use_effect_with_deps(
            move |_| {
                let is_signed = is_signed.clone();
                spawn_local(async move {
                    let result = is_signed_in("_").await.as_bool().unwrap();
                    is_signed.set(result);
                    if result {
                        history.push(AppRoute::AdminBlog);
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <Header/>
            <div class="content">
                <div id="form">
                    <h1>{ "Home" }</h1>
                    <div>
                        <input ref={mail_address_ref} type="string" />
                        <input ref={password_ref} type="password" />
                        <button onclick={onclick_sign_in}>{ format!("Add input") }</button>
                    </div>
                </div>
            </div>
            <Footer/>
        </>
    }
}
