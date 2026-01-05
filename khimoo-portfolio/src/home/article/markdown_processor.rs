use pulldown_cmark::{html, Parser};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

/// Processor for converting markdown to HTML
pub struct MarkdownProcessor;

impl MarkdownProcessor {
    /// Process markdown content and convert to HTML
    pub fn process_to_html(raw_content: &str) -> Html {
        // Convert markdown to HTML
        let parser = Parser::new(raw_content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Return as Yew HTML
        Html::from_html_unchecked(AttrValue::from(html_output))
    }
}