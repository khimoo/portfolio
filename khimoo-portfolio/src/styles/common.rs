use super::theme::Theme;

/// Common UI component styles
pub struct CommonStyles;

impl CommonStyles {
    /// Loading container with centered content
    pub fn loading_container() -> String {
        format!(
            "display: flex; justify-content: center; align-items: center; height: 100vh; background: {};",
            Theme::BACKGROUND_PRIMARY
        )
    }
    
    /// Error container with centered content
    pub fn error_container() -> String {
        format!(
            "display: flex; justify-content: center; align-items: center; height: 100vh; background: {}; color: {};",
            Theme::BACKGROUND_PRIMARY,
            Theme::ERROR_COLOR
        )
    }
    
    /// Main content container
    pub fn main_container() -> String {
        format!(
            "padding: 16px; background: {}; color: {}; min-height: 100vh;",
            Theme::BACKGROUND_PRIMARY,
            Theme::TEXT_PRIMARY
        )
    }
    
    /// Centered text container
    pub fn centered_text() -> &'static str {
        "text-align: center;"
    }
    
    /// Flex container with gap
    pub fn flex_container_with_gap(gap: &str) -> String {
        format!("display: flex; gap: {};", gap)
    }
    
    /// Full size container
    pub fn full_size() -> &'static str {
        "width: 100%; height: 100%;"
    }
}