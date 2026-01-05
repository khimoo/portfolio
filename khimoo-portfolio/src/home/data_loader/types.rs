use serde::{Deserialize, Serialize};

/// Main article data structure matching the generated JSON format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessedArticle {
    pub slug: String,
    pub title: String,
    pub metadata: ProcessedMetadata,
    pub file_path: String,
    pub outbound_links: Vec<ProcessedLink>,
    pub inbound_links: Vec<ProcessedLink>,
    pub processed_at: String,
}

/// Lightweight article data for list display (without full content)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightweightArticle {
    pub slug: String,
    pub title: String,
    pub summary: Option<String>, // First paragraph or excerpt
    pub metadata: ProcessedMetadata,
    pub file_path: String,
    pub outbound_links: Vec<ProcessedLink>,
    pub inbound_links: Vec<ProcessedLink>,
    pub processed_at: String,
}

impl From<ProcessedArticle> for LightweightArticle {
    fn from(article: ProcessedArticle) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            summary: None, // Summary will be loaded from file when needed
            metadata: article.metadata,
            file_path: article.file_path,
            outbound_links: article.outbound_links,
            inbound_links: article.inbound_links,
            processed_at: article.processed_at,
        }
    }
}

/// Article metadata structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessedMetadata {
    pub title: String,
    pub home_display: bool,
    pub category: Option<String>,
    pub importance: Option<u8>,
    pub related_articles: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub author_image: Option<String>,
}

/// Link structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessedLink {
    pub target_slug: String,
    pub link_type: LinkType,
    pub original_text: Option<String>,
}

/// Link type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkType {
    MarkdownLink,
}

/// Articles collection data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticlesData {
    pub articles: Vec<ProcessedArticle>,
    pub generated_at: String,
    pub total_count: usize,
    pub home_articles: Vec<String>,
}

/// Error types for data loading operations
#[derive(Debug, Clone, PartialEq)]
pub enum DataLoadError {
    NetworkError(String),
    ParseError(String),
    NotFound(String),
}

impl std::fmt::Display for DataLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataLoadError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DataLoadError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DataLoadError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for DataLoadError {}