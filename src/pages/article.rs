use crate::compornents::{card::Card, footer::Footer, header::Header, pagination::Pagination};
use js_bridge::fetch_article_size;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Properties};

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub page: String,
    pub article_type: String,
}

#[function_component(Article)]
pub fn article(props: &RenderedAtProps) -> Html {
    let page_clone = props.page.clone();
    let page_u8: u8 = page_clone.parse::<u8>().unwrap();
    let page_size = use_state(|| 1);
    let article_type = props.article_type.clone();

    let limit_num = 3;

    {
        let page_size = page_size.clone();
        let article_type = article_type.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let fetch_page_size = fetch_article_size(article_type)
                        .await
                        .as_f64()
                        .unwrap()
                        .round() as u8;
                    let new_page_size;
                    if fetch_page_size % limit_num != 0 {
                        new_page_size = fetch_page_size / limit_num + 2;
                    } else {
                        new_page_size = fetch_page_size / limit_num + 1;
                    }
                    page_size.set(new_page_size);
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
            <Card current_page={page_u8} limit_num={limit_num} article_type={article_type}/>
            <ul class="pagenation">
                <Pagination current_page={page_u8} page_size={*page_size}/>
            </ul>
            <Footer/>
        </>
    }
}
