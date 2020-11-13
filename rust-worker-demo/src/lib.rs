use wasm_bindgen::prelude::*;
use web_sys::Request;
use url::Url;

mod utils;
mod kv;
mod form;
mod markdown;

// This function is publicly exposed as a part of our WASM library
// It takes a request and returns a string with HTML
#[wasm_bindgen]
pub async fn main(request: Request) -> String {
    // Get the the requested path:
    // 'example.com/hello' -> '/hello'
    // and the method:
    // 'get', 'post', etc.
    let url = Url::parse(&request.url()).unwrap();
    let path = url.path().to_lowercase();
    let method = request.method().to_lowercase();

    // This is then passed into our request router, and a html response is generates
    return respond(request, method, path).await;
}

pub async fn respond(request: Request, method: String, path: String) -> String {
    let content = match path {
        // Display the rendered markdown
        p if p == "/" => match method {
            m if m == "get" => markdown::render(kv::read("content").await),
            _ => "<strong>Bad method</strong>".to_string(),
        },
        // call out to another function to handle editing the page
        p if p == "/update" => update(request, method).await,
        _ => "<strong>Bad path</strong>".to_string(),
    };

    // Format the content into a html document
    // I'm reusing some stylesheets from slightknack.dev for convinience
    // note that ideally, this would be stored in a bucket or a NS
    // and loaded at runting, rather than being hard-coded.
    format!("
        <!DOCTYPE html>
        <head>
            <link rel='stylesheet' href='https://www.slightknack.dev/static/style.css'>
            <link rel='stylesheet' href='https://fonts.googleapis.com/css?family=Overpass+Mono:400,700|Overpass:400,400i,700,700i|PT+Serif:400,400i,700,700i&display=swap'>
        </head>
        <body>
            <div class='content'>
                <center><div style='max-width: 600px;'>
                    <h1 class='title'>Rust Worker Demo</h1>
                    <p>
                        <a href='/'>Home</a> —
                        <a href='/update'>Edit</a>
                    </p>
                    <br />
                    <div align='left'>{}</div>
                </div></center>
            </div>
        </body>",
        content,
    )
}

// handle editing the page
// get request -> return a form
// post request -> update the database
pub async fn update(request: Request, method: String) -> String {
    match method {
        // get -> return a form
        m if m == "get"  => {
            // The content is a form with the old page
            let old_content = kv::read("content").await;
            format!("
                <form action='/update' method='post'>
                    <textarea
                        name='content'
                        style='resize: both;'
                        >{}</textarea>
                    <br>
                    <input type='submit' value='Update'>
                </form>",
                old_content
            )
        }
        // post -> update the database
        m if m == "post" => {
            // extract the new content from the form
            let new_content = form::extract(request).await.unwrap();
            kv::write(&"content".to_string(), &new_content).await;
            markdown::render(new_content)
        }
        _ => "<strong>Bad method on update</strong>".to_string()
    }
}

// Logging function for debugging purposes
// maps to JS's console.log
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
