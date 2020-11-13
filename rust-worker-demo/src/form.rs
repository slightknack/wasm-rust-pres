use web_sys::{Request, FormData};
use wasm_bindgen_futures::JsFuture;

// Extract the data from the editing form
pub async fn extract(request: Request) -> Option<String> {
    let js_form = JsFuture::from(request.form_data().ok()?).await.ok()?;
    let form = FormData::from(js_form);
    form.get("content").as_string()
}
