use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::types::{ArticlesData, ProcessedArticle};

/// Simple in-memory cache for article data
#[derive(Debug, Clone)]
pub struct DataCache {
    articles_cache: Rc<RefCell<Option<ArticlesData>>>,
    article_cache: Rc<RefCell<HashMap<String, ProcessedArticle>>>,
    content_cache: Rc<RefCell<HashMap<String, String>>>,
}

impl DataCache {
    pub fn new() -> Self {
        Self {
            articles_cache: Rc::new(RefCell::new(None)),
            article_cache: Rc::new(RefCell::new(HashMap::new())),
            content_cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Cache articles data
    pub fn cache_articles(&self, data: ArticlesData) {
        *self.articles_cache.borrow_mut() = Some(data);
    }

    /// Get cached articles data
    pub fn get_articles(&self) -> Option<ArticlesData> {
        self.articles_cache.borrow().clone()
    }

    /// Cache individual article
    pub fn cache_article(&self, slug: String, article: ProcessedArticle) {
        self.article_cache.borrow_mut().insert(slug, article);
    }

    /// Get cached article by slug
    pub fn get_article(&self, slug: &str) -> Option<ProcessedArticle> {
        self.article_cache.borrow().get(slug).cloned()
    }

    /// Cache article content
    pub fn cache_content(&self, file_path: String, content: String) {
        self.content_cache.borrow_mut().insert(file_path, content);
    }

    /// Get cached content by file path
    pub fn get_content(&self, file_path: &str) -> Option<String> {
        self.content_cache.borrow().get(file_path).cloned()
    }

    /// Clear all caches
    pub fn clear(&self) {
        *self.articles_cache.borrow_mut() = None;
        self.article_cache.borrow_mut().clear();
        self.content_cache.borrow_mut().clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            has_articles: self.articles_cache.borrow().is_some(),
            cached_articles_count: self.article_cache.borrow().len(),
            cached_content_count: self.content_cache.borrow().len(),
        }
    }
}

impl Default for DataCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub has_articles: bool,
    pub cached_articles_count: usize,
    pub cached_content_count: usize,
}