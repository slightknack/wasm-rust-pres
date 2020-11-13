use wasm_bindgen::prelude::*;
use web_sys::Request;
use url::Url;

mod utils;

#[wasm_bindgen]
pub fn main(request: Request) -> String {
    let url = Url::parse(&request.url()).unwrap();
    let path = url.path().to_lowercase();
    let method = request.method().to_lowercase();

    return respond(method, path);
}

pub fn respond(method: String, path: String) -> String {
    format!(
        "<center><div max-witdh='600px' text-align='left'>
            <h1>responding to: {} {}</h1>
            <p>This response was served via a WASM backend!</p>
        </div></center>",
        method,
        path,
    )
}
