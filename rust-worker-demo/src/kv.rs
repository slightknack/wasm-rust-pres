use wasm_bindgen::prelude::*;

/// bind the namespace Document to our Rust program
/// Document must match *exactly*
/// This is important
#[wasm_bindgen]
extern "C" {
    pub type Document;

    #[wasm_bindgen(static_method_of = Document)]
    pub async fn get(key: &str) -> JsValue;

    #[wasm_bindgen(static_method_of = Document)]
    pub async fn put(key: &str, value: &str) -> JsValue;
}

// Wrapper functions for manipulating the Namespace
// not strictly necessary for out current simple use-case,
// but useful in more complex situations.

pub async fn write(key: &str, value: &str) {
    Document::put(key, value).await;
}

pub async fn read(key: &str) -> String {
    Document::get(key).await.as_string().unwrap()
}
