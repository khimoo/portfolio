use std::path::PathBuf;
use std::collections::HashMap;

/// Load configuration from project.toml
pub fn load_project_config() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?
        .parent()
        .ok_or("Cannot find project root")?
        .to_path_buf();
    
    let config_path = project_root.join("project.toml");
    
    if !config_path.exists() {
        return Err("project.toml not found".into());
    }
    
    let config_content = std::fs::read_to_string(config_path)?;
    let config: toml::Value = toml::from_str(&config_content)?;
    
    let mut paths = HashMap::new();
    
    if let Some(paths_table) = config.get("paths").and_then(|v| v.as_table()) {
        for (key, value) in paths_table {
            if let Some(path_str) = value.as_str() {
                paths.insert(key.clone(), path_str.to_string());
            }
        }
    }
    
    Ok(paths)
}

/// Get default articles directory from configuration
pub fn get_default_articles_dir() -> PathBuf {
    match load_project_config() {
        Ok(config) => {
            if let Some(articles_dir) = config.get("articles_dir") {
                PathBuf::from(format!("../{}", articles_dir))
            } else {
                PathBuf::from("../content/articles")
            }
        }
        Err(_) => PathBuf::from("../content/articles")
    }
}