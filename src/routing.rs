use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/:id")]
    View { id: String },
    #[at("/admin_blog")]
    AdminBlog,
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
