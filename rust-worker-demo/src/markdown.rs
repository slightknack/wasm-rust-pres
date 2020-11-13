use pulldown_cmark::{html, Parser, Options};

// Render some raw CommonMark compliant MarkDown into html
pub fn render(raw: String) -> String {
    let parser = Parser::new_ext(&raw, Options::all());

    let mut output = String::new();
    html::push_html(&mut output, parser);
    return output;
}
