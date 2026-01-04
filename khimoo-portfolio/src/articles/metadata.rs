use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use chrono::DateTime;
use yaml_front_matter::{Document, YamlFrontMatter};

/// Article metadata structure with default values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleMetadata {
    pub title: String,
    #[serde(default)]
    pub home_display: bool,
    pub category: Option<String>,
    #[serde(default = "default_importance")]
    pub importance: u8,
    #[serde(default)]
    pub related_articles: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub author_image: Option<String>,
}

impl Default for ArticleMetadata {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            home_display: false,
            category: None,
            importance: default_importance(),
            related_articles: Vec::new(),
            tags: Vec::new(),
            created_at: None,
            updated_at: None,
            author_image: None,
        }
    }
}

fn default_importance() -> u8 {
    3
}

/// Front matter parser using yaml-front-matter library
pub struct FrontMatterParser;

impl FrontMatterParser {
    /// Parse front matter from markdown content using yaml-front-matter library
    /// Returns (metadata, remaining_content)
    pub fn parse(content: &str) -> Result<(ArticleMetadata, String)> {
        // Try to parse with yaml-front-matter
        match YamlFrontMatter::parse(content) {
            Ok(Document { metadata, content: markdown_content }) => {
                // Parse metadata into ArticleMetadata struct
                let metadata: ArticleMetadata = serde_yaml::from_value(metadata)
                    .context("Failed to deserialize front matter metadata")?;

                Ok((metadata, markdown_content))
            }
            Err(_) => {
                // No front matter found, return default metadata and full content
                Ok((ArticleMetadata::default(), content.to_string()))
            }
        }
    }

    /// Validate metadata fields
    pub fn validate_metadata(metadata: &ArticleMetadata) -> Result<()> {
        // Validate importance range
        if metadata.importance < 1 || metadata.importance > 5 {
            return Err(anyhow::anyhow!(
                "Importance must be between 1 and 5, got: {}",
                metadata.importance
            ));
        }

        // Validate title is not empty
        if metadata.title.trim().is_empty() {
            return Err(anyhow::anyhow!("Title cannot be empty"));
        }

        // Validate datetime formats if present
        if let Some(created_at) = &metadata.created_at {
            DateTime::parse_from_rfc3339(created_at)
                .context("Invalid created_at datetime format")?;
        }

        if let Some(updated_at) = &metadata.updated_at {
            DateTime::parse_from_rfc3339(updated_at)
                .context("Invalid updated_at datetime format")?;
        }

        Ok(())
    }
}