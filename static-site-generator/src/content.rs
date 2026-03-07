use anyhow::{Context, Result};
use chrono::NaiveDate;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub layout: Option<String>,
    pub date: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Content {
    pub frontmatter: Frontmatter,
    pub body: String,
    pub html: String,
    pub path: PathBuf,
    pub category: Option<String>,
    pub date: Option<NaiveDate>,
    pub slug: String,
    pub url: String,
}

impl Content {
    pub fn from_file(path: &Path, base_content_dir: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {:?}", path))?;

        let is_html = path.extension().map_or(false, |ext| ext == "html");

        let (frontmatter, body) = parse_frontmatter(&content)?;

        // For HTML files, use content as-is; for markdown, convert to HTML
        let html = if is_html {
            body.clone()
        } else {
            markdown_to_html(&body)
        };

        // Extract category and date from path
        let relative_path = path.strip_prefix(base_content_dir)
            .unwrap_or(path);

        let (category, date, slug) = extract_metadata(path, relative_path)?;

        Ok(Self {
            frontmatter,
            body,
            html,
            path: path.to_path_buf(),
            category,
            date,
            slug,
            url: String::new(), // Will be set by generator
        })
    }
}

fn parse_frontmatter(content: &str) -> Result<(Frontmatter, String)> {
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$").unwrap();

    if let Some(caps) = re.captures(content) {
        let yaml = &caps[1];
        let body = caps[2].to_string();

        let frontmatter: Frontmatter = serde_yaml::from_str(yaml)
            .with_context(|| "Failed to parse YAML frontmatter")?;

        Ok((frontmatter, body))
    } else {
        // No frontmatter, use defaults
        Ok((
            Frontmatter {
                title: "Untitled".to_string(),
                layout: None,
                date: None,
                extra: HashMap::new(),
            },
            content.to_string(),
        ))
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn extract_metadata(
    full_path: &Path,
    relative_path: &Path,
) -> Result<(Option<String>, Option<NaiveDate>, String)> {
    let components: Vec<_> = relative_path.components().collect();

    // Check if in posts/ directory
    let category = if components.len() > 2 && components[0].as_os_str() == "posts" {
        Some(components[1].as_os_str().to_string_lossy().to_string())
    } else {
        None
    };

    // Extract date and slug from filename (e.g., "2024-07-18-i-survived-covid.md")
    let filename = full_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled");

    let date_re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})-(.+)$").unwrap();

    let (date, slug) = if let Some(caps) = date_re.captures(filename) {
        let year: i32 = caps[1].parse()?;
        let month: u32 = caps[2].parse()?;
        let day: u32 = caps[3].parse()?;
        let slug = caps[4].to_string();

        let date = NaiveDate::from_ymd_opt(year, month, day)
            .with_context(|| format!("Invalid date: {}-{}-{}", year, month, day))?;

        (Some(date), slug)
    } else {
        (None, filename.to_string())
    };

    Ok((category, date, slug))
}
