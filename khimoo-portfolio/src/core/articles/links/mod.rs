pub mod extractor;
pub mod validator;

// Re-export types
pub use extractor::{ExtractedLink, LinkType, LinkExtractor};
pub use validator::{
    LinkValidator, ValidationReport, ValidationError,
    ValidationErrorType, ValidationSummary, ProcessedArticleRef
};