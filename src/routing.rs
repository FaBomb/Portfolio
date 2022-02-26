use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/admin_blog/:s")]
    AdminBlogPage,
    #[at("view/:id")]
    View { id: String },
    #[at("/admin_work")]
    AdminWork,
    #[at("/admin_article_edit")]
    AdminArticleEdit,
    #[at("/works")]
    Works,
    #[at("/profile")]
    Profile,
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}
#[derive(Clone, Routable, PartialEq)]
pub enum AdminBlogRoute {
    #[at("/admin_blog/")]
    AdminBlog,
    #[not_found]
    #[at("/admin_blog/404")]
    NotFound,
}
