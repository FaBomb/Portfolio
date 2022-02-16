use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
mod admin;
mod compornents;
mod pages;
mod routing;

use admin::{
    admin::Admin, admin_article_edit::AdminArticleEdit, admin_blog::AdminBlog,
    admin_work::AdminWork,
};
use pages::{
    blog::Blog, home::Home, notfound::NotFound, profile::Profile, view::View, works::Works,
};
use routing::AppRoute;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Blog => html! { <Blog /> },
        AppRoute::View { id } => html! { <View id={id.to_string()}/> },
        AppRoute::AdminBlog => html! { <AdminBlog/> },
        AppRoute::AdminWork => html! { <AdminWork /> },
        AppRoute::AdminArticleEdit => html! { <AdminArticleEdit /> },
        AppRoute::Profile => html! { <Profile /> },
        AppRoute::Works => html! { <Works /> },
        AppRoute::Admin => html! { <Admin /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

#[wasm_bindgen]
pub fn start(mode: AppMode) {
    let log_level = match mode {
        AppMode::Dev => log::Level::Trace,
        AppMode::Production => log::Level::Error,
    };
    wasm_logger::init(wasm_logger::Config::new(log_level));
    yew::start_app::<Main>();
}

pub fn main() {
    panic!()
}

#[wasm_bindgen]
pub enum AppMode {
    Dev,
    Production,
}
