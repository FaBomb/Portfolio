use crate::compornents::{footer::Footer, header::Header};
use crate::routing::AppRoute;
use js_bridge::is_signed_in;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state};
use yew_router::prelude::*;

#[function_component(AdminWork)]
pub fn admin_work() -> Html {
    let history = use_history().unwrap();
    let is_signed = use_state(|| true);

    {
        let is_signed = is_signed.clone();
        use_effect_with_deps(
            move |_| {
                let is_signed = is_signed.clone();
                spawn_local(async move {
                    let result = is_signed_in("_").await.as_bool().unwrap();
                    is_signed.set(result);
                    if !result {
                        history.push(AppRoute::Admin);
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
            <h1>{ "Admin Blog" }</h1>
            <Footer/>
        </>
    }
}
