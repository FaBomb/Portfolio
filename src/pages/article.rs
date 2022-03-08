use crate::compornents::{card::Card, footer::Footer, header::Header, pagination::Pagination};
use js_bridge::{fetch_article_size, is_signed_in};
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
    let is_signed = use_state(|| false);

    let limit_num = 6;

    {
        let page_size = page_size.clone();
        let article_type = article_type.clone();
        let is_signed = is_signed.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let result = is_signed_in("_").await.as_bool().unwrap();
                    is_signed.set(result);
                    let fetch_page_size = fetch_article_size(article_type, result)
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

    let prop_article_type = article_type.clone();
    html! {
        <>
            <Header/>
            <div class="article">
                <h1>{"- "} { article_type.clone() } {" -"}</h1>
                <div class="cards">
                    <Card current_page={page_u8} limit_num={limit_num} article_type={article_type} is_signed={*is_signed}/>
                </div>
                <ul class="pagination">
                    <Pagination article_type={prop_article_type} current_page={page_u8} page_size={*page_size}/>
                </ul>
            </div>
            <Footer/>
        </>
    }
}
