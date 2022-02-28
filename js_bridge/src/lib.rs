use wasm_bindgen::prelude::*;
use web_sys::File;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name = "sign_in", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn sign_in(email: &str, password: &str) -> JsValue;

    #[wasm_bindgen(js_name = "sign_out", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn sign_out(_: &str) -> JsValue;

    #[wasm_bindgen(js_name = "is_signed_in", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn is_signed_in(_: &str) -> JsValue;

    #[wasm_bindgen(js_name = "upload", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn upload(files: File) -> JsValue;

    #[wasm_bindgen(js_name = "del_from_url", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn del_from_url(url: &str) -> JsValue;

    #[wasm_bindgen(js_name = "set_content", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn set_content(collection: String, article: String);

    #[wasm_bindgen(js_name = "set_category", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn set_category(category: String);

    #[wasm_bindgen(js_name = "set_tag", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn set_tag(tag: String);

    #[wasm_bindgen(js_name = "fetch_categories", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn fetch_categories() -> JsValue;

    #[wasm_bindgen(js_name = "fetch_article_contents", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn fetch_article_contents(collection: String, index: u8, limit_num: u8) -> JsValue;

    #[wasm_bindgen(js_name = "fetch_article_size", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn fetch_article_size(collection: String) -> JsValue;

    #[wasm_bindgen(js_name = "fetch_article_content_from_id", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn fetch_article_content_from_id(collection: String, id: String) -> JsValue;

    #[wasm_bindgen(js_name = "fetch_all_article_content_from_id", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn fetch_all_article_content_from_id(collection: String, id: String) -> JsValue;

    #[wasm_bindgen(js_name = "fetch_tags", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn fetch_tags() -> JsValue;

    #[wasm_bindgen(js_name = "del_category", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn del_category(category: String);

    #[wasm_bindgen(js_name = "del_tag", js_namespace = ["window", "_wasm_js_bridge"])]
    pub async fn del_tag(tag: String);

}
