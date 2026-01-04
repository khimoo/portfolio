pub mod home;
pub mod config;

// Only include articles module for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
pub mod articles;

// Re-export commonly used types (only for non-WASM)
#[cfg(not(target_arch = "wasm32"))]
pub use articles::{
    ArticleMetadata, 
    ExtractedLink, 
    LinkType, 
    LinkExtractor,
    FrontMatterParser,
    ArticleProcessor
};