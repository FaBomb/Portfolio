use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/admin/:s")]
    AdminPage,
    #[at("/admin_blog/:s")]
    AdminBlogPage,
    #[at("/admin_work/:s")]
    AdminWorkPage,
    #[at("/blog/:s")]
    BlogPage,
    #[at("/work/:s")]
    WorkPage,
    #[at("view/:id")]
    View { id: String },
    #[at("/profile")]
    Profile,
    #[not_found]
    #[at("/404")]
    NotFound,
}
#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin/entrance")]
    Admin,
    #[not_found]
    #[at("/admin/404")]
    NotFound,
}
#[derive(Clone, Routable, PartialEq)]
pub enum AdminBlogRoute {
    #[at("/admin_blog/:id")]
    AdminArticleEdit { id: String },
    #[not_found]
    #[at("/admin_blog/404")]
    NotFound,
}
#[derive(Clone, Routable, PartialEq)]
pub enum AdminWorkRoute {
    #[at("/admin_work/:id")]
    AdminArticleEdit { id: String },
    #[not_found]
    #[at("/admin_work/404")]
    NotFound,
}
#[derive(Clone, Routable, PartialEq)]
pub enum BlogRoute {
    #[at("/blog/:page")]
    Blog { page: String },
    #[not_found]
    #[at("/blog/404")]
    NotFound,
}
#[derive(Clone, Routable, PartialEq)]
pub enum WorkRoute {
    #[at("/work/:page")]
    Work { page: String },
    #[not_found]
    #[at("/work/404")]
    NotFound,
}
