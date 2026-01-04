use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;

use khimoo_portfolio::articles::{
    ArticleProcessor, LinkValidator
};
use khimoo_portfolio::articles::links::ValidationReportFormatter;
use khimoo_portfolio::config_loader::get_default_articles_dir;

#[derive(Parser)]
#[command(name = "validate_links")]
#[command(about = "Validate links in markdown articles")]
struct Args {
    /// Directory containing markdown articles
    #[arg(short, long)]
    articles_dir: Option<PathBuf>,
    
    /// Output directory for validation reports
    #[arg(short, long, default_value = "validation_reports")]
    output_dir: PathBuf,
    
    /// Output format (console, json, files)
    #[arg(short, long, default_value = "console")]
    format: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Use configuration default if not provided
    let articles_dir = args.articles_dir.unwrap_or_else(|| get_default_articles_dir());
    
    // Initialize article processor
    let processor = ArticleProcessor::new()?;
    
    // Process all articles in the directory
    let mut processed_articles = Vec::new();
    
    if articles_dir.exists() {
        for entry in std::fs::read_dir(&articles_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let content = std::fs::read_to_string(&path)?;
                match processor.process_article(&path, &content) {
                    Ok(article) => processed_articles.push(article),
                    Err(e) => eprintln!("Failed to process {}: {}", path.display(), e),
                }
            }
        }
    }
    
    // Validate links
    let validator = LinkValidator::new(&processed_articles);
    let report = validator.validate_all()?;
    
    // Output report based on format
    match args.format.as_str() {
        "json" => {
            let json_output = ValidationReportFormatter::format_json(&report)?;
            println!("{}", json_output);
        }
        "files" => {
            ValidationReportFormatter::write_report_files(&report, &args.output_dir)?;
            println!("Reports written to {}", args.output_dir.display());
        }
        _ => {
            let console_output = ValidationReportFormatter::format_console(&report);
            println!("{}", console_output);
        }
    }
    
    // Exit with error code if there are validation errors
    if report.summary.broken_links > 0 || report.summary.invalid_references > 0 {
        std::process::exit(1);
    }
    
    Ok(())
}