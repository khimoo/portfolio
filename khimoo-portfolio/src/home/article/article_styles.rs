use crate::styles::Theme;

/// Styles specific to article display
pub struct ArticleStyles;

impl ArticleStyles {
    /// Get base article styles
    pub fn base_styles() -> &'static str {
        r#"
        .markdown-body { 
            line-height: 1.6; 
            color: #e0e0e0; 
        }
        .markdown-body h1, .markdown-body h2, .markdown-body h3 { 
            margin-top: 24px; 
            margin-bottom: 16px; 
            color: #e0e0e0; 
        }
        .markdown-body p { 
            margin-bottom: 16px; 
            color: #e0e0e0; 
        }
        .markdown-body ul, .markdown-body ol { 
            margin-bottom: 16px; 
            padding-left: 30px; 
            color: #e0e0e0; 
        }
        .markdown-body code { 
            background: #2d3748; 
            color: #e0e0e0; 
            padding: 2px 4px; 
            border-radius: 3px; 
            font-size: 85%; 
        }
        .markdown-body pre { 
            background: #2d3748; 
            color: #e0e0e0; 
            padding: 16px; 
            border-radius: 6px; 
            overflow: auto; 
        }
        .markdown-body blockquote { 
            border-left: 4px solid #66b3ff; 
            padding-left: 16px; 
            color: #aaa; 
            margin: 0 0 16px 0; 
        }
        .markdown-body a { 
            color: #66b3ff; 
            text-decoration: none; 
        }
        .markdown-body a:hover { 
            color: #99ccff; 
            text-decoration: underline; 
        }
        "#
    }

    /// Get unified article container styles (for both header and content)
    pub fn unified_article_container() -> String {
        format!(
            "padding: 16px; max-width: 800px; margin: 0 auto; background: {}; min-height: 100vh;",
            Theme::BACKGROUND_PRIMARY
        )
    }

    /// Get integrated article header styles (within content container)
    pub fn integrated_article_header() -> String {
        format!(
            "margin-bottom: 32px; padding-bottom: 16px; border-bottom: 1px solid #444; \
             display: flex; justify-content: space-between; align-items: flex-start; gap: 20px;"
        )
    }

    /// Get integrated article title styles (consistent with content)
    pub fn integrated_article_title() -> String {
        format!(
            "margin: 0 0 16px 0; font-size: 2.5em; color: {}; font-weight: bold;",
            Theme::TEXT_PRIMARY
        )
    }

    /// Get integrated article metadata styles (consistent with content)
    pub fn integrated_article_meta() -> String {
        format!(
            "font-size: 14px; color: {}; display: flex; gap: 16px; flex-wrap: wrap;",
            Theme::TEXT_SECONDARY
        )
    }

    /// Get author image container styles
    pub fn author_image_container() -> &'static str {
        "flex-shrink: 0; display: flex; align-items: stretch;"
    }

    /// Get author image styles
    pub fn author_image() -> &'static str {
        "height: 120px; object-fit: cover;"
    }

    /// Get tag styles
    pub fn tag_style() -> &'static str {
        "background: #4a5568; color: #e0e0e0; padding: 2px 6px; border-radius: 3px; font-size: 12px;"
    }

    /// Get related articles footer styles
    pub fn related_articles_footer() -> &'static str {
        "margin-top: 48px; padding-top: 24px; border-top: 1px solid #444;"
    }

    /// Get related articles list styles
    pub fn related_articles_list() -> &'static str {
        "list-style: none; padding: 0;"
    }

    /// Get related articles item styles
    pub fn related_articles_item() -> &'static str {
        "margin-bottom: 8px;"
    }

    // Legacy methods for backward compatibility (will be removed after migration)
    /// @deprecated Use unified_article_container() instead
    pub fn article_container() -> String {
        Self::unified_article_container()
    }

    /// @deprecated Use integrated_article_header() instead
    pub fn article_header() -> String {
        Self::integrated_article_header()
    }

    /// @deprecated Use integrated_article_title() instead
    pub fn article_title() -> String {
        Self::integrated_article_title()
    }

    /// @deprecated Use integrated_article_meta() instead
    pub fn article_meta() -> String {
        Self::integrated_article_meta()
    }

    /// Get article index container styles
    pub fn index_container() -> String {
        format!(
            "padding: 16px; background: {}; color: {}; min-height: 100vh;",
            Theme::BACKGROUND_PRIMARY,
            Theme::TEXT_PRIMARY
        )
    }

    /// Get article list item styles
    pub fn list_item() -> &'static str {
        "margin-bottom: 20px; padding: 16px; border-radius: 8px;"
    }
}