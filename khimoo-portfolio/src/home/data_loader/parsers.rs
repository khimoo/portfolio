/// Simple front matter parser for WASM environment
pub struct FrontMatterParser;

impl FrontMatterParser {
    /// Parse front matter from markdown content
    /// Returns (remaining_content) - metadata is stripped
    pub fn parse_content_only(content: &str) -> String {
        let content = content.trim();
        
        // Check if content starts with front matter delimiter
        if content.starts_with("---") {
            let lines: Vec<&str> = content.lines().collect();
            
            // Find the closing delimiter
            let mut end_index = None;
            for (i, line) in lines.iter().enumerate().skip(1) {
                if line.trim() == "---" {
                    end_index = Some(i);
                    break;
                }
            }
            
            if let Some(end_idx) = end_index {
                // Return content after the closing delimiter
                let remaining_lines = &lines[end_idx + 1..];
                return remaining_lines.join("\n").trim_start().to_string();
            }
        }
        
        // No front matter found, return original content
        content.to_string()
    }
}

/// Utility for extracting article summaries
pub struct SummaryExtractor;

impl SummaryExtractor {
    /// Extract summary from article content
    pub fn extract_summary(content: &str) -> String {
        // Remove markdown headers and get first paragraph
        let lines: Vec<&str> = content.lines().collect();
        let mut summary_lines = Vec::new();
        let mut found_content = false;

        for line in lines {
            let trimmed = line.trim();

            // Skip empty lines and headers at the beginning
            if trimmed.is_empty() || trimmed.starts_with('#') {
                if found_content {
                    break; // Stop at first empty line or header after content
                }
                continue;
            }

            found_content = true;
            summary_lines.push(trimmed);

            // Stop after first paragraph or when we have enough content
            if summary_lines.join(" ").len() > 200 {
                break;
            }
        }

        let summary = summary_lines.join(" ");
        if summary.len() > 200 {
            format!("{}...", &summary[..197])
        } else {
            summary
        }
    }
}