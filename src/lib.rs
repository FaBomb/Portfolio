use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
mod admin;
mod compornents;
mod pages;
mod routing;

use admin::{admin::Admin, admin_article_edit::AdminArticleEdit};
use pages::{article::Article, home::Home, notfound::NotFound, profile::Profile, view::View};
use routing::{AdminRoute, AppRoute, BlogRoute, WorkRoute};

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
        AppRoute::View { id } => html! { <View id={id.to_string()}/> },
        AppRoute::AdminPage => {
            html! { <Switch<AdminRoute> render={Switch::render(switch_admin)} /> }
        }
        AppRoute::BlogPage => {
            html! { <Switch<BlogRoute> render={Switch::render(switch_blog)} /> }
        }
        AppRoute::WorkPage => {
            html! { <Switch<WorkRoute> render={Switch::render(switch_work)} /> }
        }
        AppRoute::Profile => html! { <Profile /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

fn switch_admin(route: &AdminRoute) -> Html {
    match route {
        AdminRoute::Admin => html! { <Admin /> },
        AdminRoute::AdminArticleEdit => html! { <AdminArticleEdit /> },
        AdminRoute::NotFound => html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        },
    }
}
fn switch_blog(route: &BlogRoute) -> Html {
    match route {
        BlogRoute::Blog { page } => {
            html! {<Article page={page.to_string()} article_type={"blog".to_string()}/>}
        }
        BlogRoute::NotFound => html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        },
    }
}
fn switch_work(route: &WorkRoute) -> Html {
    match route {
        WorkRoute::Work { page } => {
            html! {<Article page={page.to_string()} article_type={"work".to_string()}/>}
        }
        WorkRoute::NotFound => html! {
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
