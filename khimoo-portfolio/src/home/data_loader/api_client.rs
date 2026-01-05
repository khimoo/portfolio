use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use crate::config::{get_config, AppConfig};
use super::types::{ArticlesData, ProcessedArticle, LightweightArticle, DataLoadError};
use super::parsers::FrontMatterParser;

/// HTTP client for loading article data
#[derive(Debug, Clone)]
pub struct DataLoader {
    config: &'static AppConfig,
}

impl DataLoader {
    pub fn new() -> Self {
        Self {
            config: get_config(),
        }
    }

    /// Load articles data with error handling and fallback
    pub async fn load_articles(&self) -> Result<ArticlesData, DataLoadError> {
        let url = self.config.data_url("articles.json");

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("DataLoader: Loading articles from: {}", url).into());

        match self.fetch_json::<ArticlesData>(&url).await {
            Ok(data) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("DataLoader: Successfully loaded {} articles", data.articles.len()).into());
                Ok(data)
            },
            Err(e) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::warn_1(&format!("Failed to load articles data: {}", e).into());

                // Fallback to empty data structure
                Ok(ArticlesData {
                    articles: Vec::new(),
                    generated_at: "1970-01-01T00:00:00Z".to_string(),
                    total_count: 0,
                    home_articles: Vec::new(),
                })
            }
        }
    }

    /// Load lightweight articles data (without full content)
    pub async fn load_lightweight_articles(&self) -> Result<Vec<LightweightArticle>, DataLoadError> {
        let articles_data = self.load_articles().await?;
        let lightweight_articles = articles_data.articles
            .into_iter()
            .map(LightweightArticle::from)
            .collect();
        Ok(lightweight_articles)
    }

    /// Load full article content from file path
    pub async fn load_article_content(&self, file_path: &str) -> Result<String, DataLoadError> {
        let url = self.config.article_url(file_path);

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("DataLoader: Loading article content from: {}", url).into());

        let content = self.fetch_text(&url).await?;
        Ok(content)
    }

    /// Load article content without front matter metadata (content only)
    pub async fn load_article_content_only(&self, file_path: &str) -> Result<String, DataLoadError> {
        let full_content = self.load_article_content(file_path).await?;
        let content_only = FrontMatterParser::parse_content_only(&full_content);

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"DataLoader: Successfully separated content from metadata".into());

        Ok(content_only)
    }

    /// Load article by slug (metadata only, content must be loaded separately)
    pub async fn load_article_by_slug(&self, slug: &str) -> Result<ProcessedArticle, DataLoadError> {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("DataLoader: Looking for article with slug: {}", slug).into());

        let articles_data = self.load_articles().await?;

        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(&format!("DataLoader: Loaded {} articles", articles_data.articles.len()).into());
            for article in &articles_data.articles {
                web_sys::console::log_1(&format!("DataLoader: Available slug: '{}'", article.slug).into());
            }
        }

        let found_article = articles_data.articles
            .into_iter()
            .find(|article| {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("DataLoader: Comparing '{}' with '{}'", article.slug, slug).into());
                article.slug == slug
            });

        match found_article {
            Some(article) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("DataLoader: Found article: {}", article.title).into());
                Ok(article)
            }
            None => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("DataLoader: Article not found: {}", slug).into());
                Err(DataLoadError::NotFound(format!("Article not found: {}", slug)))
            }
        }
    }

    /// Generic JSON fetching method
    async fn fetch_json<T>(&self, url: &str) -> Result<T, DataLoadError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self.fetch_response(url).await?;

        let json = JsFuture::from(response.json().map_err(|e| {
            DataLoadError::ParseError(format!("Failed to get JSON: {:?}", e))
        })?)
        .await
        .map_err(|e| DataLoadError::ParseError(format!("Failed to parse JSON: {:?}", e)))?;

        let data: T = serde_wasm_bindgen::from_value(json)
            .map_err(|e| DataLoadError::ParseError(format!("Failed to deserialize: {:?}", e)))?;

        Ok(data)
    }

    /// Generic text fetching method
    async fn fetch_text(&self, url: &str) -> Result<String, DataLoadError> {
        let response = self.fetch_response(url).await?;

        let text = JsFuture::from(response.text().map_err(|e| {
            DataLoadError::ParseError(format!("Failed to get text: {:?}", e))
        })?)
        .await
        .map_err(|e| DataLoadError::ParseError(format!("Failed to parse text: {:?}", e)))?;

        let content = text.as_string()
            .ok_or_else(|| DataLoadError::ParseError("Response is not a string".to_string()))?;

        Ok(content)
    }

    /// Generic HTTP response fetching
    async fn fetch_response(&self, url: &str) -> Result<Response, DataLoadError> {
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)
            .map_err(|e| DataLoadError::NetworkError(format!("Failed to create request: {:?}", e)))?;

        let window = web_sys::window()
            .ok_or_else(|| DataLoadError::NetworkError("No window object".to_string()))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| DataLoadError::NetworkError(format!("Fetch failed: {:?}", e)))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|e| DataLoadError::NetworkError(format!("Invalid response: {:?}", e)))?;

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("DataLoader: Response status: {} {}", resp.status(), resp.status_text()).into());

        if !resp.ok() {
            return Err(DataLoadError::NotFound(format!(
                "HTTP {}: {}",
                resp.status(),
                resp.status_text()
            )));
        }

        Ok(resp)
    }
}

impl Default for DataLoader {
    fn default() -> Self {
        Self::new()
    }
}
