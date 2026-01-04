use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use regex::Regex;

/// Types of links that can be extracted from markdown content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkType {
    WikiLink,      // [[article-name]] format
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
    wiki_regex: Regex,
    markdown_regex: Regex,
}

impl LinkExtractor {
    /// Create a new link extractor with compiled regex patterns
    pub fn new() -> Result<Self> {
        let wiki_regex = Regex::new(r"\[\[([^\]]+)\]\]")
            .context("Failed to compile wiki link regex")?;
        let markdown_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)")
            .context("Failed to compile markdown link regex")?;

        Ok(Self {
            wiki_regex,
            markdown_regex,
        })
    }

    /// Extract all links from markdown content
    pub fn extract_links(&self, content: &str) -> Vec<ExtractedLink> {
        let mut links = Vec::new();

        // Extract wiki-style links [[article-name]] or [[target|display]]
        for cap in self.wiki_regex.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let inner = cap.get(1).unwrap().as_str(); // 内部の文字列 (例: "target" または "target|display")

            // '|' があれば左をリンクターゲット、右を表示テキストとして扱う
            let parts: Vec<&str> = inner.splitn(2, '|').collect();
            let link_target = parts[0].trim();
            // Optional: display_text を取り出したい場合は parts.get(1).map(|s| s.trim().to_string())
            // let display_text = if parts.len() == 2 { Some(parts[1].trim().to_string()) } else { None };

            links.push(ExtractedLink {
                target_slug: self.generate_slug_from_title(link_target),
                link_type: LinkType::WikiLink,
                original_text: full_match.as_str().to_string(),
            });
        }

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

    /// Generate a slug from article title (for wiki links)
    fn generate_slug_from_title(&self, title: &str) -> String {
        let slug = title
            .to_lowercase()
            .trim()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .trim_matches('-')
            .to_string();

        // Replace multiple consecutive dashes with single dash
        let re = Regex::new(r"-+").unwrap();
        re.replace_all(&slug, "-").to_string()
    }
}

impl Default for LinkExtractor {
    fn default() -> Self {
        Self::new().expect("Failed to create default LinkExtractor")
    }
}