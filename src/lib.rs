use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
mod admin;
mod compornents;
mod pages;
mod routing;

use admin::{admin::Admin, admin_article_edit::AdminArticleEdit};
use pages::{article::Article, home::Home, notfound::NotFound, profile::Profile, view::View};
use routing::{
    AdminBlogRoute, AdminRoute, AdminWorkRoute, AppRoute, BlogRoute, ViewBlogRoute, ViewWorkRoute,
    WorkRoute,
};

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
        AppRoute::ViewBlogPage => {
            html! { <Switch<ViewBlogRoute> render={Switch::render(switch_view_blog)} /> }
        }
        AppRoute::ViewWorkPage => {
            html! { <Switch<ViewWorkRoute> render={Switch::render(switch_view_work)} /> }
        }
        AppRoute::AdminPage => {
            html! { <Switch<AdminRoute> render={Switch::render(switch_admin)} /> }
        }
        AppRoute::AdminBlogPage => {
            html! { <Switch<AdminBlogRoute> render={Switch::render(switch_admin_blog)} /> }
        }
        AppRoute::AdminWorkPage => {
            html! { <Switch<AdminWorkRoute> render={Switch::render(switch_admin_work)} /> }
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

fn switch_view_blog(route: &ViewBlogRoute) -> Html {
    match route {
        ViewBlogRoute::View { id } => html! { <View id={id.to_string()}/> },
        ViewBlogRoute::NotFound => html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        },
    }
}
fn switch_view_work(route: &ViewWorkRoute) -> Html {
    match route {
        ViewWorkRoute::View { id } => html! { <View id={id.to_string()}/> },
        ViewWorkRoute::NotFound => html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        },
    }
}
fn switch_admin(route: &AdminRoute) -> Html {
    match route {
        AdminRoute::Admin => html! { <Admin /> },
        AdminRoute::NotFound => html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        },
    }
}
fn switch_admin_blog(route: &AdminBlogRoute) -> Html {
    match route {
        AdminBlogRoute::AdminArticleEdit { id } => {
            html! { <AdminArticleEdit id={id.to_string()}/> }
        }
        AdminBlogRoute::NotFound => html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        },
    }
}
fn switch_admin_work(route: &AdminWorkRoute) -> Html {
    match route {
        AdminWorkRoute::AdminArticleEdit { id } => {
            html! { <AdminArticleEdit id={id.to_string()}/> }
        }
        AdminWorkRoute::NotFound => html! {
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
            html! {<Article page={page.to_string()} article_type={"works".to_string()}/>}
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
