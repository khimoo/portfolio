/// Application theme constants and utilities
pub struct Theme;

impl Theme {
    // Color palette
    pub const BACKGROUND_PRIMARY: &'static str = "#081D35";
    pub const TEXT_PRIMARY: &'static str = "#e0e0e0";
    pub const TEXT_SECONDARY: &'static str = "#aaa";
    pub const ACCENT_BLUE: &'static str = "#66b3ff";
    pub const ACCENT_BLUE_HOVER: &'static str = "#99ccff";
    pub const ERROR_COLOR: &'static str = "#ff6b6b";
    pub const SUCCESS_COLOR: &'static str = "#4CAF50";
    
    // Node colors
    pub const NODE_DEFAULT: &'static str = "slateblue";
    pub const NODE_BORDER: &'static str = "rgba(0,0,0,0.2)";
    
    // Layout
    pub const CONTAINER_MAX_WIDTH: &'static str = "1000px";
    pub const BORDER_RADIUS: &'static str = "8px";
    pub const BORDER_RADIUS_SMALL: &'static str = "4px";
    
    // Animation
    pub const TRANSITION_FAST: &'static str = "0.2s ease-in-out";
    pub const TRANSITION_NORMAL: &'static str = "0.3s ease-in-out";
    
    /// Get base CSS styles for the application
    pub fn base_styles() -> &'static str {
        r#"
        html, body {
            background: var(--bg-color, #081D35);
            color: var(--text-color, #e0e0e0);
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }
        
        :root {
            --bg-color: #081D35;
            --text-color: #e0e0e0;
            --link-color: #66b3ff;
            --meta-color: #aaa;
            --error-color: #ff6b6b;
            --success-color: #4CAF50;
        }
        
        @media (prefers-color-scheme: light) {
            :root {
                --bg-color: #ffffff;
                --text-color: #333333;
                --link-color: #007bff;
                --meta-color: #666;
            }
        }
        
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
        "#
    }
    
    /// Get loading spinner styles
    pub fn loading_spinner_style() -> String {
        format!(
            "border: 4px solid #444; border-top: 4px solid {}; border-radius: 50%; width: 40px; height: 40px; animation: spin 2s linear infinite; margin: 0 auto;",
            Self::ACCENT_BLUE
        )
    }
    
    /// Get button styles
    pub fn button_style() -> String {
        format!(
            "padding: 8px 16px; background: {}; color: white; border: none; border-radius: {}; cursor: pointer; transition: {};",
            Self::ACCENT_BLUE,
            Self::BORDER_RADIUS_SMALL,
            Self::TRANSITION_FAST
        )
    }
}