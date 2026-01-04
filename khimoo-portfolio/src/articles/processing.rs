use anyhow::Result;
use std::path::Path;

use crate::articles::{
    metadata::FrontMatterParser,
    links::{LinkExtractor, ProcessedArticleRef}
};

/// High-level article processing functionality
pub struct ArticleProcessor {
    link_extractor: LinkExtractor,
}

impl ArticleProcessor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            link_extractor: LinkExtractor::new()?,
        })
    }

    /// Process a single article file and return processed article reference
    pub fn process_article(&self, file_path: &Path, content: &str) -> Result<ProcessedArticleRef> {
        // Parse front matter and content
        let (metadata, markdown_content) = FrontMatterParser::parse(content)?;
        
        // Validate metadata
        FrontMatterParser::validate_metadata(&metadata)?;
        
        // Extract links from content
        let outbound_links = self.link_extractor.extract_links(&markdown_content);
        
        // Generate slug from file path
        let slug = self.generate_slug_from_path(file_path);
        
        Ok(ProcessedArticleRef {
            slug,
            title: metadata.title.clone(),
            metadata,
            outbound_links,
            inbound_links: Vec::new(), // Will be populated later during validation
            file_path: file_path.to_string_lossy().to_string(),
        })
    }

    /// Generate slug from file path
    fn generate_slug_from_path(&self, file_path: &Path) -> String {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string()
    }
}

impl Default for ArticleProcessor {
    fn default() -> Self {
        Self::new().expect("Failed to create default ArticleProcessor")
    }
}