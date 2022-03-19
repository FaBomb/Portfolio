use crate::compornents::{card::Card, footer::Footer, header::Header};
use crate::routing::{AppRoute, BlogRoute, WorkRoute};
use yew::{function_component, html};
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let go_blog = {
        let history = history.clone();
        move |_| {
            history.push(BlogRoute::Blog {
                page: "1".to_string(),
            })
        }
    };
    let go_work = {
        let history = history.clone();
        move |_| {
            history.push(WorkRoute::Work {
                page: "1".to_string(),
            })
        }
    };
    let go_profile = {
        let history = history.clone();
        move |_| history.push(AppRoute::Profile)
    };

    html! {
        <>
            <Header/>
            <div class="home-ex">
                <div class="ex-me">
                    <h2>{ "Tech" }</h2>
                    <h3>{ "×" }</h3>
                    <h2>{ "Life" }</h2>
                    <h3>{ "=" }</h3>
                </div>
                <div class="ex-detail">
                    <p>
                        {"This website is FaBomb's portfolio site. "}<br/>
                        {"I will keep a record of my work and learning."}
                    </p>
                </div>
            </div>
            <div class="home-work">
                <h2><a onclick={go_work.clone()}>{ "- Works -" }</a></h2>
                <p class="small-text sub-text">{"過去に制作した作品"}</p>
                <div class="cards">
                    <Card current_page={1} limit_num={4} article_type={"works"}
                    is_signed={false} query_content={"".to_string()} />
                </div>
                <a onclick={go_work} class="detail">{"more"}</a>
            </div>
            <div class="home-blog">
                <h2><a onclick={go_blog.clone()}>{ "- Blog -" }</a></h2>
                <p class="small-text sub-text">{"ブログ記事"}</p>
                <div class="cards">
                    <Card current_page={1} limit_num={4} article_type={"blog"}
                    is_signed={false} query_content={"".to_string()}/>
                </div>
                <a onclick={go_blog} class="detail">{"more"}</a>
            </div>
            <div class="home-profile">
                <h2><a onclick={go_profile.clone()}>{ "- Profile -" }</a></h2>
                <p class="small-text sub-text">{"プロフィール"}</p>
                <div class="profile-box">
                    <img src="images/profile.jpg" alt="プロフィール画像" />
                    <dl>
                        <dt>{"Name"}</dt>
                        <dd>{"Yuta Toyomi"}</dd>
                        <dt>{"Birthday"}</dt>
                        <dd>{"12.31.1997"}</dd>
                        <dt>{"Job"}</dt>
                        <dd>{"Web Engineer"}</dd>
                    </dl>
                </div>
                <a onclick={go_profile} class="detail">{"more"}</a>
            </div>
            <Footer/>
        </>
    }
}
