use std::collections::HashMap;

use khimoo_portfolio::{
    ArticleMetadata,
    ExtractedLink,
    FrontMatterParser,
    LinkExtractor,
    LinkType,
};
use khimoo_portfolio::{
    LinkValidator,
    ProcessedArticleRef,
    ValidationError,
    ValidationErrorType,
    ValidationReport,
    ValidationReportFormatter,
    ValidationSummary,
};

#[test]
fn test_parse_complete_front_matter() {
    let content = r#"---
title: "Test Article"
home_display: true
category: "programming"
importance: 4
related_articles: ["article1", "article2"]
tags: ["rust", "test"]
created_at: "2024-01-01T00:00:00Z"
updated_at: "2024-01-02T00:00:00Z"
---

# Test Content

This is the markdown content.
"#;

    let (metadata, markdown_content) = FrontMatterParser::parse(content).unwrap();
    
    assert_eq!(metadata.title, "Test Article");
    assert_eq!(metadata.home_display, true);
    assert_eq!(metadata.category, Some("programming".to_string()));
    assert_eq!(metadata.importance, 4);
    assert_eq!(metadata.related_articles, vec!["article1", "article2"]);
    assert_eq!(metadata.tags, vec!["rust", "test"]);
    assert_eq!(metadata.created_at, Some("2024-01-01T00:00:00Z".to_string()));
    assert_eq!(metadata.updated_at, Some("2024-01-02T00:00:00Z".to_string()));
    
    assert!(markdown_content.contains("# Test Content"));
    assert!(markdown_content.contains("This is the markdown content."));
}

#[test]
fn test_parse_minimal_front_matter() {
    let content = r#"---
title: "Minimal Article"
---

Just some content.
"#;

    let (metadata, markdown_content) = FrontMatterParser::parse(content).unwrap();
    
    assert_eq!(metadata.title, "Minimal Article");
    assert_eq!(metadata.home_display, false); // default
    assert_eq!(metadata.category, None);
    assert_eq!(metadata.importance, 3); // default
    assert!(metadata.related_articles.is_empty());
    assert!(metadata.tags.is_empty());
    
    assert!(markdown_content.contains("Just some content."));
}

#[test]
fn test_parse_no_front_matter() {
    let content = "# Just Markdown\n\nNo front matter here.";
    
    let (metadata, markdown_content) = FrontMatterParser::parse(content).unwrap();
    
    // Should use defaults
    assert_eq!(metadata.title, "Untitled");
    assert_eq!(metadata.home_display, false);
    assert_eq!(metadata.importance, 3);
    
    // Content should be unchanged
    assert_eq!(markdown_content, content);
}

#[test]
fn test_parse_tags_from_front_matter() {
    let content = r#"---
title: "Tagged Article"
tags: ["rust", "programming", "web"]
---

Content here.
"#;

    let (metadata, _) = FrontMatterParser::parse(content).unwrap();
    
    assert_eq!(metadata.tags, vec!["rust", "programming", "web"]);
}

#[test]
fn test_validate_metadata_valid() {
    let metadata = ArticleMetadata {
        title: "Valid Article".to_string(),
        home_display: true,
        category: Some("programming".to_string()),
        importance: 4,
        related_articles: vec![],
        tags: vec!["rust".to_string()],
        created_at: Some("2024-01-01T00:00:00Z".to_string()),
        updated_at: None,
        author_image: None,
    };
    
    assert!(FrontMatterParser::validate_metadata(&metadata).is_ok());
}

#[test]
fn test_validate_metadata_invalid_importance() {
    let metadata = ArticleMetadata {
        title: "Test".to_string(),
        importance: 6, // Invalid: should be 1-5
        ..Default::default()
    };
    
    assert!(FrontMatterParser::validate_metadata(&metadata).is_err());
}

#[test]
fn test_validate_metadata_empty_title() {
    let metadata = ArticleMetadata {
        title: "   ".to_string(), // Empty after trim
        ..Default::default()
    };
    
    assert!(FrontMatterParser::validate_metadata(&metadata).is_err());
}

#[test]
fn test_validate_metadata_invalid_datetime() {
    let metadata = ArticleMetadata {
        title: "Test".to_string(),
        created_at: Some("invalid-date".to_string()),
        ..Default::default()
    };
    
    assert!(FrontMatterParser::validate_metadata(&metadata).is_err());
}

#[test]
fn test_extract_markdown_links() {
    let extractor = LinkExtractor::new().unwrap();
    let content = "Check out [this article](article-slug) and [another one](second-slug).";
    
    let links = extractor.extract_links(content);
    
    assert_eq!(links.len(), 2);
    
    assert_eq!(links[0].target_slug, "article-slug");
    assert_eq!(links[0].link_type, LinkType::MarkdownLink);
    assert_eq!(links[0].original_text, "[this article](article-slug)");
    
    assert_eq!(links[1].target_slug, "second-slug");
    assert_eq!(links[1].link_type, LinkType::MarkdownLink);
    assert_eq!(links[1].original_text, "[another one](second-slug)");
}

#[test]
fn test_extract_mixed_links() {
    let extractor = LinkExtractor::new().unwrap();
    let content = r#"
        Start with [markdown link](slug-here) and [another link](second-slug).
        Also [external link](https://example.com).
        "#;
    
    let links = extractor.extract_links(content);
    
    // Should extract 2 links (excluding external http link)
    assert_eq!(links.len(), 2);
    
    // Check they are in order of appearance
    assert_eq!(links[0].link_type, LinkType::MarkdownLink);
    assert_eq!(links[0].target_slug, "slug-here");
    
    assert_eq!(links[1].link_type, LinkType::MarkdownLink);
    assert_eq!(links[1].target_slug, "second-slug");
}

#[test]
fn test_ignore_external_links() {
    let extractor = LinkExtractor::new().unwrap();
    let content = r#"
        Check out [external site](https://example.com) and [email](mailto:test@example.com).
        But also [internal link](internal-slug).
        "#;
    
    let links = extractor.extract_links(content);
    
    // Should only extract the internal link
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].target_slug, "internal-slug");
    assert_eq!(links[0].link_type, LinkType::MarkdownLink);
}

#[test]
fn test_real_article_patterns() {
    let extractor = LinkExtractor::new().unwrap();
    
    // Test pattern from rust-async.md
    let content = r#"
        非同期プログラミングを理解するには、まず[Tokio基礎](tokio-basics)を理解することから始めましょう。
        
        実用的な[パターン集](async-patterns)も参考になります。
        
        [Hello記事](hello)でも触れましたが、非同期処理は重要です。
        "#;
    
    let links = extractor.extract_links(content);
    
    assert_eq!(links.len(), 3);
    
    // Check specific patterns
    assert_eq!(links[0].target_slug, "tokio-basics");
    assert_eq!(links[0].link_type, LinkType::MarkdownLink);
    
    assert_eq!(links[1].target_slug, "async-patterns");
    assert_eq!(links[1].link_type, LinkType::MarkdownLink);
    
    assert_eq!(links[2].target_slug, "hello");
    assert_eq!(links[2].link_type, LinkType::MarkdownLink);
}

#[test]
fn test_broken_link_patterns() {
    let extractor = LinkExtractor::new().unwrap();
    
    // Test pattern from broken-link-test.md
    let content = r#"
        - [存在しない記事](存在しない記事)へのリンク
        - [壊れたリンク](broken-slug)へのmarkdownリンク
        "#;
    
    let links = extractor.extract_links(content);
    
    assert_eq!(links.len(), 2);
    
    assert_eq!(links[0].target_slug, "存在しない記事");
    assert_eq!(links[0].link_type, LinkType::MarkdownLink);
    
    assert_eq!(links[1].target_slug, "broken-slug");
    assert_eq!(links[1].link_type, LinkType::MarkdownLink);
}

#[test]
fn test_edge_cases() {
    let extractor = LinkExtractor::new().unwrap();
    
    // Empty content
    assert_eq!(extractor.extract_links("").len(), 0);
    
    // Malformed links
    let malformed = "[incomplete link and [incomplete](";
    assert_eq!(extractor.extract_links(malformed).len(), 0);
    
    // Nested brackets (should work fine with markdown links)
    let nested = "[outer [inner] link](target-slug)";
    let links = extractor.extract_links(nested);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].target_slug, "target-slug");
}

#[test]
fn test_link_validator_broken_links() {
    let articles = vec![
        ProcessedArticleRef {
            slug: "article1".to_string(),
            title: "Article 1".to_string(),
            outbound_links: vec![
                ExtractedLink {
                    target_slug: "article2".to_string(),
                    link_type: LinkType::MarkdownLink,
                    original_text: "[article2](article2)".to_string(),
                },
                ExtractedLink {
                    target_slug: "nonexistent".to_string(),
                    link_type: LinkType::MarkdownLink,
                    original_text: "[nonexistent](nonexistent)".to_string(),
                },
            ],
            inbound_links: vec![],
            metadata: ArticleMetadata::default(),
        },
        ProcessedArticleRef {
            slug: "article2".to_string(),
            title: "Article 2".to_string(),
            outbound_links: vec![],
            inbound_links: vec![],
            metadata: ArticleMetadata::default(),
        },
    ];

    let validator = LinkValidator::new(&articles);
    let report = validator.validate();

    assert_eq!(report.errors.len(), 1);
    assert_eq!(report.errors[0].error_type, ValidationErrorType::BrokenLink);
    assert_eq!(report.errors[0].target_slug, "nonexistent");
    assert_eq!(report.errors[0].source_article, "article1");
}

// Additional tests for validation, reporting, etc. can be added here
// following the same pattern but without WikiLink references