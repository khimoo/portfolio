use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use regex::Regex;

/// Types of links that can be extracted from markdown content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkType {
    MarkdownLink,  // [text](slug) format
}

/// Represents a link found in markdown content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedLink {
    pub target_slug: String,
    pub link_type: LinkType,
    pub original_text: String,
}

/// Link extractor for markdown content
pub struct LinkExtractor {
    markdown_regex: Regex,
}

impl LinkExtractor {
    /// Create a new link extractor with compiled regex patterns
    pub fn new() -> Result<Self> {
        let markdown_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)")
            .context("Failed to compile markdown link regex")?;

        Ok(Self {
            markdown_regex,
        })
    }

    /// Extract all links from markdown content
    pub fn extract_links(&self, content: &str) -> Vec<ExtractedLink> {
        let mut links = Vec::new();

        // Extract markdown-style links [text](slug)
        for cap in self.markdown_regex.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let _text = cap.get(1).unwrap().as_str();
            let target = cap.get(2).unwrap().as_str();

            // Only process internal links (not starting with http/https)
            if !target.starts_with("http") && !target.starts_with("mailto:") {
                links.push(ExtractedLink {
                    target_slug: target.to_string(),
                    link_type: LinkType::MarkdownLink,
                    original_text: full_match.as_str().to_string(),
                });
            }
        }

        links
    }
}

impl Default for LinkExtractor {
    fn default() -> Self {
        Self::new().expect("Failed to create default LinkExtractor")
    }
}