use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PageData {
    pub url: String,
    pub title: String,
    pub meta_description: Option<String>,
    pub headings: Vec<String>,
    pub links: Vec<String>,
}
