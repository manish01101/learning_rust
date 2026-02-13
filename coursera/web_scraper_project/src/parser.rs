use crate::models::PageData;
use scraper::{Html, Selector};

pub fn extract_details(url: &str, html_content: &str) -> PageData {
    // Parse the HTML document
    let document = Html::parse_document(html_content);

    // 1. Extract Title
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|element| element.text().collect::<String>())
        .unwrap_or_else(|| "No Title".to_string());

    // 2. Extract Meta Description
    let meta_selector = Selector::parse("meta[name='description']").unwrap();
    let meta_description = document
        .select(&meta_selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .map(|s| s.to_string());

    // 3. Extract Headings (h1, h2, h3)
    let heading_selector = Selector::parse("h1, h2, h3").unwrap();
    let headings: Vec<String> = document
        .select(&heading_selector)
        .map(|element| element.text().collect::<String>().trim().to_string())
        .filter(|h| !h.is_empty())
        .collect();

    // 4. Extract Links
    let link_selector = Selector::parse("a[href]").unwrap();
    let links: Vec<String> = document
        .select(&link_selector)
        .filter_map(|element| element.value().attr("href"))
        .map(|href| href.to_string())
        .collect();

    PageData {
        url: url.to_string(),
        title,
        meta_description,
        headings,
        links,
    }
}
