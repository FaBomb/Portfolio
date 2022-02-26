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
use routing::{AdminBlogRoute, AppRoute};

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
        AppRoute::AdminBlogPage => {
            html! { <Switch<AdminBlogRoute> render={Switch::render(switch_admin_blog)} /> }
        }
        AppRoute::AdminWork => html! { <AdminWork /> },
        AppRoute::AdminArticleEdit => html! { <AdminArticleEdit /> },
        AppRoute::Profile => html! { <Profile /> },
        AppRoute::Works => html! { <Works /> },
        AppRoute::Admin => html! { <Admin /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

fn switch_admin_blog(route: &AdminBlogRoute) -> Html {
    match route {
        AdminBlogRoute::AdminBlog => html! {<h1>{"Profile"}</h1>},
        AdminBlogRoute::NotFound => html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        },
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

#[wasm_bindgen]
pub enum AppMode {
    Dev,
    Production,
}
