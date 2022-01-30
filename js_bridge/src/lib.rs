use wasm_bindgen::prelude::*;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use names::{Generator, Name};

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name = "get_store", js_namespace = ["window", "_wasm_js_bridge"])]
    fn alert(s: &str);

}

pub fn test_connect(s: &str) {
    alert(s);
}