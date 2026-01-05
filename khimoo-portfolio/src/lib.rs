pub mod home;
pub mod config;
pub mod styles;

// Only include articles module for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
pub mod articles;

// Only include config_loader for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
pub mod config_loader;

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