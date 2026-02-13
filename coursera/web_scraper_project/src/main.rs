mod models;
mod parser;

use anyhow::{Context, Result};
use futures::stream::{self, StreamExt}; // Ensure StreamExt is imported
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

// Concurrency Config
const CONCURRENT_REQUESTS: usize = 5;

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();

    let urls = vec![
        "https://www.rust-lang.org",
        "https://en.wikipedia.org/wiki/Rust_(programming_language)",
        "https://github.com",
        "https://stackoverflow.com",
        "https://www.mozilla.org",
    ];

    println!("Starting scraper on {} URLs...", urls.len());

    let client = Client::builder().user_agent("RustScraper/1.0").build()?;

    // 1. Create the stream of futures
    let mut bodies = stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            tokio::spawn(async move { process_url(client, url).await })
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    let mut results = Vec::new();

    // Use a while let loop instead of for_each
    // This allows us to modify 'results' without closure lifetime issues
    while let Some(b) = bodies.next().await {
        match b {
            Ok(Ok(data)) => {
                println!("Successfully scraped: {}", data.url);
                results.push(data);
            }
            Ok(Err(e)) => eprintln!("Failed to scrape a page: {}", e),
            Err(e) => eprintln!("Task Join Error: {}", e),
        }
    }

    // Output logic
    save_to_json(&results, "scraped_data.json")?;

    println!(
        "Done! Scraped {} pages in {:.2?}.",
        results.len(),
        start_time.elapsed()
    );

    Ok(())
}

async fn process_url(client: Client, url: &str) -> Result<models::PageData> {
    let resp = client
        .get(url)
        .send()
        .await
        .context("Failed to send request")?;

    if !resp.status().is_success() {
        anyhow::bail!("Request failed with status: {}", resp.status());
    }

    let html = resp.text().await.context("Failed to read response body")?;
    let data = parser::extract_details(url, &html);

    Ok(data)
}

fn save_to_json(data: &Vec<models::PageData>, filename: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    println!("Data saved to {}", filename);
    Ok(())
}
