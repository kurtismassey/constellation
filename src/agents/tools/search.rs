use crate::config::Settings;
use anyhow::Result;
use html2text::from_read;
use reqwest;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub url: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct SearchArgs {
    pub query: String,
    pub num_results: Option<u32>,
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Failed to perform web search: {0}")]
    SearchError(String),
    #[error("Failed to fetch content: {0}")]
    FetchError(String),
}

#[derive(Serialize, Deserialize)]
pub struct WebSearch;

impl Tool for WebSearch {
    const NAME: &'static str = "web_search";
    type Error = SearchError;
    type Args = SearchArgs;
    type Output = Vec<SearchResult>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "web_search".to_string(),
            description: "Search the web and fetch relevant content".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query to perform"
                    },
                    "num_results": {
                        "type": "number",
                        "description": "Number of results to fetch (default: 3)",
                        "default": 3
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        tracing::info!("calling web_search tool");

        let num_results = args.num_results.unwrap_or(3);

        let client = reqwest::Client::new();
        let settings = Settings::new();

        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&num={}",
            settings.google_api_key, settings.google_search_cx, args.query, num_results
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|error| SearchError::SearchError(error.to_string()))?;

        let search_results: serde_json::Value = response
            .json()
            .await
            .map_err(|error| SearchError::SearchError(error.to_string()))?;

        let urls = search_results["items"]
            .as_array()
            .ok_or_else(|| SearchError::SearchError("No search results found".to_string()))?
            .iter()
            .filter_map(|item| item["link"].as_str().map(String::from))
            .collect::<Vec<String>>();

        let mut results = Vec::new();
        for url in urls {
            match fetch_web_content(&url).await {
                Ok(content) => {
                    tracing::info!("Fetched content from {}", &url);
                    let truncated = content.chars().take(2000).collect::<String>();
                    results.push(SearchResult {
                        url: url.clone(),
                        content: truncated,
                    })
                }
                Err(e) => {
                    return Err(SearchError::FetchError(format!(
                        "Failed to fetch {}: {}",
                        url, e
                    )));
                }
            }
        }

        Ok(results)
    }
}

async fn fetch_web_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?;
    let html = response.text().await?;
    let text = from_read(Cursor::new(html), 80);

    Ok(text?)
}
